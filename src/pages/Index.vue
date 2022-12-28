<template>
  <div class="outer-box">
    <div class="options-column">
      <img class="mb-3 about-logo-redirect" :src="HorizontalLogo" width="200" @click="openAboutPage" />
      <v-btn class="mt-6" size="large" rounded="lg" :prepend-icon="mdiFileImage" :disabled="isProcessing" elevation="0"
        @click="openImage">
        Select Images
      </v-btn>
      <UpscaleTypeOption :disabled="isProcessing" class="mt-2 mb-5" @upscale-type-changed="setUpscaleType" />
      <v-divider class="mb-10" />
      <!-- Scale factor seems not to be working -->
      <!-- <UpscaleFactorOptions @upscale-factor-changed="updateUpscaleFactor" /> -->
      <v-btn size="large" rounded="lg" class="mb-2" :disabled="isReadyToUpscale" elevation="0" width="310"
        @click="startProcessing">
        {{
    isMultipleFiles ? "Upscale Selected Images" : "Upscale Selected Image"
}}
      </v-btn>
      <v-btn size="large" rounded="lg" :disabled="isProcessing" elevation="0" @click="clearSelectedImage">
        Clear
      </v-btn>
      <div class="d-flex">
        <v-btn elevation="0" class="config-button" size="32" :icon="mdiMenu" @click="openConfig"></v-btn>
      </div>
    </div>
    <div class="image-area mt-5" :class="{ 'text-center': !isMultipleFiles }">
      <h5 class="mb-2 path-text" v-if="imagePath">{{ imagePath }}</h5>
      <h5 class="mb-2 path-text" :key="imagePath.path" v-for="imagePath in imagePaths">
        <v-progress-circular v-if="!imagePath.isReady" v-show="showMultipleFilesProcessingIcon" indeterminate
          color="#ff7a00" size="16" />
        <v-icon v-else size="16" :icon="mdiImageCheck" v-show="showMultipleFilesProcessingIcon" />
        <span class="ml-2">{{ imagePath.path }}</span>
        <v-divider />
      </h5>
      <v-progress-circular class="loading-gif" color="#ff7a00" indeterminate :size="128" :width="12"
        v-if="isProcessing && !isMultipleFiles" />
      <div class="file-drop-area mt-8" v-if="!imageBlob && !imagePaths.length" @click="openImage">
        Click to select images or drop them here
      </div>
      <ImagePreviewer :images="[upscaledImageBlob, imageBlob]" :is-processing="isProcessing" v-if="imageBlob" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import HorizontalLogo from '@/assets/upscale-rs-horizontal.png';
import UpscaleTypeOption from "@/components/UpscaleTypeOption.vue";
import ImagePreviewer from "@/components/ImagePreviewer.vue";
import { mdiFileImage, mdiImageCheck, mdiMenu } from "@mdi/js";
import { invoke } from "@tauri-apps/api/tauri";
import { loadImage } from "@/helpers/loadImageBase64"
import { sendTauriNotification } from "@/helpers/tauriNotification"
import { listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";
import { WebviewWindow } from "@tauri-apps/api/window";

interface ImagePathsDisplay {
  path: string;
  isReady: boolean;
}

type UpscaleType = "general" | "digital";
type UpscaleFactor = "2" | "3" | "4";

const isProcessing = ref(false);
const imagePath = ref("");
const imagePaths: Ref<ImagePathsDisplay[]> = ref([]);
const imageBlob = ref("");
const upscaledImageBlob = ref("");
const upscaleFactor: Ref<UpscaleFactor> = ref("4");
const upscaleType: Ref<UpscaleType> = ref("general");
const isMultipleFiles = ref(false);
const showMultipleFilesProcessingIcon = ref(false);

// Computes if the user is ready to upscale the image. Used the simplify the DOM code.
const isReadyToUpscale = computed(() => {
  return !(
    (imagePath.value || imagePaths.value.length > 0) &&
    !isProcessing.value
  );
});

/**
 * Listens for file drops on the window and decides if is a single or multiple file upload.
 */
listen("tauri://file-drop", async (event) => {
  const files = event.payload as string[];
  if (!files.length || isProcessing.value) {
    return;
  }
  clearSelectedImage();
  if (files.length > 1) {
    showMultipleFilesProcessingIcon.value = false;
    isMultipleFiles.value = true;
    imagePaths.value = files
      .map((file) => {
        return {
          path: file,
          isReady: false,
        };
      })
      .filter((file) => {
        return (
          file.path.endsWith(".png") ||
          file.path.endsWith(".jpg") ||
          file.path.endsWith(".jpeg")
        );
      });
  } else {
    const image = files[0]
    isMultipleFiles.value = false;
    if (
      !(
        image.endsWith(".png") ||
        image.endsWith(".jpg") ||
        image.endsWith(".jpeg")
      )
    ) {
      alert("Please select a valid image file.");
      return;
    }
    try {
      imageBlob.value = await loadImage(image);
      imagePath.value = image;
    } catch (err: any) {
      await invoke("write_log", {
        message: `Error reading image: ${err}`,
      });
    }
  }
});

function openAboutPage() {
  // https://tauri.app/v1/guides/features/multiwindow#create-a-window-in-javascript
  const webview = new WebviewWindow("about-page", {
    height: 400,
    width: 500,
    title: "About",
    url: "/about",
  });
  // since the webview window is created asynchronously,
  // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
  webview.once("tauri://created", function () {
    // webview window successfully created
  });
  webview.once("tauri://error", function (err) {
    console.error(err);
    // an error happened creating the webview window
  });
}

/**
 * Sets the upscale type.
 *
 * @param value - The upscale type. Available values are `general` and `digital`.
 */
function setUpscaleType(value: UpscaleType) {
  upscaleType.value = value;
}

function openConfig() {
  // https://tauri.app/v1/guides/features/multiwindow#create-a-window-in-javascript
  const webview = new WebviewWindow("config-page", {
    height: 400,
    width: 500,
    title: "Config",
    url: "/config",
  });
  // since the webview window is created asynchronously,
  // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
  webview.once("tauri://created", function () {
    // webview window successfully created
  });
  webview.once("tauri://error", function (err) {
    console.error(err);
    // an error happened creating the webview window
  });
}

/**
 * Clears the selected image and some other variables.
 */
function clearSelectedImage() {
  imagePath.value = "";
  imagePaths.value = [];
  imageBlob.value = "";
  upscaledImageBlob.value = "";
  showMultipleFilesProcessingIcon.value = false;
  isMultipleFiles.value = false;
}

/**
 * Sets the upscale factor. Currently it's not working.
 */
// function updateUpscaleFactor(value: UpscaleFactor) {
//   upscaleFactor.value = value;
// }

/**
 * Opens the image file dialog from Tauri.
 *
 * It is used in the single and multiple file selector.
 */
async function openImage() {
  // Open a selection dialog for image files
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "",
        extensions: ["png", "jpeg", "jpg"],
      },
    ],
  });
  if (Array.isArray(selected) && selected.length > 1) {
    clearSelectedImage();
    isMultipleFiles.value = true;
    showMultipleFilesProcessingIcon.value = false;
    imagePaths.value = selected.map((path) => {
      return {
        path,
        isReady: false,
      };
    });
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    clearSelectedImage();
    isMultipleFiles.value = false;
    imagePath.value = selected[0];
    try {
      imageBlob.value = await loadImage(imagePath.value);
    } catch (err: any) {
      await invoke("write_log", { message: err.toString() });
      alert(err);
    }
  }
}

/**
 * Runs the correct single or multiple file processing function.
 *
 * It is used to control the code flow.
 */
function startProcessing() {
  if (isMultipleFiles.value) {
    upscaleMultipleImages();
  } else {
    upscaleSingleImage();
  }
}

/**
 * Upscales multiple images function.
 *
 * It will ask the user to select a folder to save the upscaled images.
 *
 * It will update the `isReady` property of the `imagePaths` array to true when the image is ready.
 */
async function upscaleMultipleImages() {
  const outputFolder = await open({
    directory: true,
  });
  if (outputFolder === null) {
    return;
  }
  isProcessing.value = true;
  showMultipleFilesProcessingIcon.value = true;
  try {
    for (let i = 0; i < imagePaths.value.length; i++) {
      let outputFile: string = await invoke("replace_file_suffix", {
        path: imagePaths.value[i].path,
      });

      outputFile = `${outputFolder}/${outputFile.split("/").pop()}`;
      await invoke("upscale_single_image", {
        path: imagePaths.value[i].path,
        savePath: outputFile,
        upscaleFactor: upscaleFactor.value,
        upscaleType: upscaleType.value,
      });
      imagePaths.value[i].isReady = true;
      sendTauriNotification("Upscale-rs", "All images have been upscaled successfully!");
    }
  } catch (err: any) {
    showMultipleFilesProcessingIcon.value = false;
    await invoke("write_log", { message: err.toString() });
    alert(err);
  } finally {
    isProcessing.value = false;
  }
}

/**
 * Upscales a single image.
 *
 * It will ask the user to select a file name and location to save the upscaled image.
 *
 * After the image is upscaled, it will send a `alert` to the user.
 */
async function upscaleSingleImage() {
  if (imagePath.value === "") {
    alert("No image selected");
    return;
  }
  const imageSavePath = await save({
    defaultPath: await invoke("replace_file_suffix", { path: imagePath.value }),
  });
  if (imageSavePath === null) {
    // user cancelled the selection
    return;
  }
  isProcessing.value = true;
  try {
    const output = await invoke("upscale_single_image", {
      path: imagePath.value,
      savePath: imageSavePath,
      upscaleFactor: upscaleFactor.value,
      upscaleType: upscaleType.value,
    });
    sendTauriNotification("Upscale-rs", "The image was upscaled successfully!");

    // Load the upscaled image
    try {
      upscaledImageBlob.value = await loadImage(imageSavePath);
    } catch (err: any) {
      await invoke("write_log", { message: err.toString() });
    }
  } catch (err: any) {
    await invoke("write_log", { message: err.toString() });
    alert(err);
  } finally {
    isProcessing.value = false;
  }
}
</script>

<style scoped lang="scss">
.loading-gif {
  z-index: 1;
  margin-left: -70px;
  margin-top: 190px;
  position: fixed;
}

.image-area {
  min-width: 500px;
  min-height: 500px;
}

.path-text {
  font-size: 14px;
  font-weight: normal;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-drop-area {
  min-width: 500px;
  min-height: 500px;
  border: 2px dashed rgba($color: #969696, $alpha: 0.4);
  border-radius: 24px;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  cursor: pointer;
}

.outer-box {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  width: 800px;
  height: 100%;
}

.about-logo-redirect {
  margin-left: 2px;
  margin-bottom: 0px !important;
  height: 30px;
  cursor: pointer;
}

.config-button {
  margin-top: 160px;
}

.options-column {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: center;
  width: 100%;
  height: 100%;
  padding: 20px;
  box-sizing: border-box;
}
</style>
