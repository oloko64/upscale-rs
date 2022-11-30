<template>
  <div class="outer-box">
    <div class="options-column">
      <v-switch
        v-model="isMultipleFiles"
        inset
        hide-details
        :disabled="isProcessing"
        label="Multiple Images"
      ></v-switch>
      <v-btn
        size="large"
        rounded="lg"
        :prepend-icon="mdiFileImage"
        :disabled="isProcessing"
        elevation="0"
        @click="openImage"
      >
        {{ isMultipleFiles ? "Select Images" : "Select Image" }}
      </v-btn>
      <UpscaleTypeOption
        :disabled="isProcessing"
        class="mt-2 mb-5"
        @upscale-type-changed="setUpscaleType"
      />
      <v-divider class="mb-10" />
      <!-- Scale factor seems not to be working -->
      <!-- <UpscaleFactorOptions @upscale-factor-changed="updateUpscaleFactor" /> -->
      <v-btn
        size="large"
        rounded="lg"
        class="mb-2"
        :disabled="isReadyToUpscale"
        elevation="0"
        width="310"
        @click="startProcessing"
      >
        {{
          isMultipleFiles ? "Upscale Selected Images" : "Upscale Selected Image"
        }}
      </v-btn>
      <v-btn
        size="large"
        rounded="lg"
        :disabled="isProcessing"
        elevation="0"
        @click="clearSelectedImage"
      >
        Clear
      </v-btn>
      <img class="mb-3 about-logo-redirect" src="../assets/upscale-rs-horizontal.png" width="200" @click="openAboutPage" />
    </div>
    <div class="image-area mt-5" :class="{ 'text-center': !isMultipleFiles }">
      <h4 class="mb-2">{{ imagePath }}</h4>
      <h4 class="mb-2" :key="imagePath.path" v-for="imagePath in imagePaths">
        {{ imagePath.path }}
        <v-progress-circular
          v-if="!imagePath.isReady"
          v-show="showMultipleFilesProcessingIcon"
          indeterminate
          color="primary"
          size="16"
        />
        <v-icon
          v-else
          size="16"
          :icon="mdiImageCheck"
          v-show="showMultipleFilesProcessingIcon"
        />
        <v-divider />
      </h4>
      <v-progress-circular
        class="loading-gif"
        color="primary"
        indeterminate
        :size="128"
        :width="12"
        v-if="isProcessing && !isMultipleFiles"
      />
      <v-img
        class="image-src"
        :src="imageBlob"
        width="500"
        height="500"
        aspect-ratio="1"
        cover
        v-if="!!imageBlob"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, Ref, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";
import UpscaleTypeOption from "../components/UpscaleTypeOption.vue";
import { mdiFileImage, mdiImageCheck, mdiTimelineQuestion } from "@mdi/js";
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

// Watch for the switch between single and multiple files and run the function that clear some variables.
watch(isMultipleFiles, () => {
  clearSelectedImage();
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
    alert(err);
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

/**
 * Clears the selected image and some other variables.
 */
function clearSelectedImage() {
  imagePath.value = "";
  imagePaths.value = [];
  imageBlob.value = "";
  showMultipleFilesProcessingIcon.value = false;
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
    multiple: isMultipleFiles.value,
    filters: [
      {
        name: "",
        extensions: ["png", "jpeg", "jpg"],
      },
    ],
  });
  if (Array.isArray(selected)) {
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
    imagePath.value = selected;
    try {
      const imageBytes = await invoke("read_image_base64", { path: selected });
      imageBlob.value = `data:image/png;base64,${imageBytes}`;
    } catch (err) {
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
      let outputFile: string = await invoke('replace_file_suffix', { path: imagePaths.value[i].path })

      outputFile = `${outputFolder}/${outputFile.split("/").pop()}`;
      await invoke("upscale_single_image", {
        path: imagePaths.value[i].path,
        savePath: outputFile,
        upscaleFactor: upscaleFactor.value,
        upscaleType: upscaleType.value,
      });
      imagePaths.value[i].isReady = true;
    }
  } catch (err) {
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
    defaultPath: await invoke('replace_file_suffix', { path: imagePath.value })
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
    alert(output);
  } catch (err) {
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
.image-src {
  border-radius: 24px;
  border: 2px solid rgba($color: #969696, $alpha: 0.4);
}
.image-area {
  min-width: 500px;
  min-height: 500px;
}
.outer-box {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  width: 800px;
  height: 100%;
}

.about-logo-redirect {
  margin-top: 161px;
  margin-bottom: 0px !important;
  cursor: pointer;
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
