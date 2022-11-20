<template>
  <div class="outer-box">
    <div class="options-column">
      <v-switch
        v-model="isMultipleFiles"
        inset
        hide-details
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
      <UpscaleTypeOption class="mt-2 mb-5" @upscale-type-changed="setUpscaleType" />
      <v-divider class="mb-10"/>
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
        <v-divider/>
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
import UpscaleTypeOption from "./UpscaleTypeOption.vue";
import { mdiFileImage, mdiImageCheck } from "@mdi/js";

interface ImagePathsDisplay {
  path: string;
  isReady: boolean;
}

const isProcessing = ref(false);
const imagePath = ref("");
const imagePaths: Ref<ImagePathsDisplay[]> = ref([]);
const imageBlob = ref("");
const upscaleFactor: Ref<"2" | "3" | "4"> = ref("4");
const upscaleType: Ref<"general" | "digital"> = ref("general");
const isMultipleFiles = ref(false);
const showMultipleFilesProcessingIcon = ref(false);

const isReadyToUpscale = computed(() => {
  return !(
    (imagePath.value || imagePaths.value.length > 0) &&
    !isProcessing.value
  );
});

watch(isMultipleFiles, () => {
  clearSelectedImage();
});

function setUpscaleType(value: any) {
  upscaleType.value = value;
}

function clearSelectedImage() {
  imagePath.value = "";
  imagePaths.value = [];
  imageBlob.value = "";
  showMultipleFilesProcessingIcon.value = false;
}

// function updateUpscaleFactor(value: any) {
//   upscaleFactor.value = value.target.value;
// }

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

function startProcessing() {
  if (isMultipleFiles.value) {
    upscaleMultipleImages();
  } else {
    upscaleSingleImage();
  }
}

async function upscaleMultipleImages() {
  isProcessing.value = true;
  const outputFolder = await open({
    directory: true,
  });
  showMultipleFilesProcessingIcon.value = true;
  try {
    for (let i = 0; i < imagePaths.value.length; i++) {
      let upscaledFilePath = "";
      if (imagePaths.value[i].path.endsWith(".png")) {
        upscaledFilePath = imagePaths.value[i].path.replace(
          new RegExp(".png" + "$"),
          "_upscaled-4x.png"
        );
      } else if (imagePaths.value[i].path.endsWith(".jpg")) {
        upscaledFilePath = imagePaths.value[i].path.replace(
          new RegExp(".jpg" + "$"),
          "_upscaled-4x.jpg"
        );
      } else if (imagePaths.value[i].path.endsWith(".jpeg")) {
        upscaledFilePath = imagePaths.value[i].path.replace(
          new RegExp(".jpeg" + "$"),
          "_upscaled-4x.jpeg"
        );
      }
      const outputFile = `${outputFolder}/${upscaledFilePath.split("/").pop()}`;
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

async function upscaleSingleImage() {
  if (imagePath.value === "") {
    alert("No image selected");
    return;
  }
  const imageSavePath = await save({
    defaultPath: imagePath.value,
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
