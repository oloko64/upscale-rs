<template>
  <div class="outer-box">
    <div class="options-column">
      <button
        :disabled="isProcessing"
        :class="{ 'blocked-cursor': isProcessing }"
        type="button"
        @click="openImage"
      >
        Select Image
      </button>
      <button
        :disabled="isProcessing"
        :class="{ 'blocked-cursor': isProcessing }"
        type="button"
        @click="upscaleSingleImage"
      >
        Upscale Selected Image
      </button>
      <UpscaleFactorOptions @upscale-factor-changed="updateUpscaleFactor" />
      <button
        :disabled="isProcessing"
        :class="{ 'blocked-cursor': isProcessing }"
        type="button"
        @click="clearSelectedImage"
      >
        Clear
      </button>
    </div>
    <div class="image-area">
      <h4>{{ imagePath }}</h4>
      <img
        class="loading-gif"
        src="../assets/loading-gif.gif"
        width="180"
        height="180"
        v-if="isProcessing"
      />
      <img
        class="image-src"
        :src="imageBlob"
        width="500"
        height="500"
        v-if="!!imageBlob"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";
import UpscaleFactorOptions from "./UpscaleFactorOptions.vue";

const isProcessing = ref(false);
const imagePath = ref("");
const imageBlob = ref("");
const upscaleFactor = ref("4");

function clearSelectedImage() {
  imagePath.value = "";
  imageBlob.value = "";
}

function updateUpscaleFactor(value: any) {
  upscaleFactor.value = value.target.value;
}

async function openImage() {
  // Open a selection dialog for image files
  const selected = await open({
    filters: [
      {
        name: "",
        extensions: ["png", "jpeg", "jpg"],
      },
    ],
  });
  if (Array.isArray(selected)) {
    // user selected multiple files
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    imagePath.value = selected;
    imageBlob.value = `data:image/png;base64,${await invoke(
      "read_image_base64",
      { path: selected }
    )}`;
  }
}

async function upscaleSingleImage() {
  if (imagePath.value === "") {
    alert("No image selected");
    return;
  }
  isProcessing.value = true;
  const imageSavePath = await save({
    defaultPath: imagePath.value,
  });
  const output = await invoke("upscale_single_image", {
    path: imagePath.value,
    savePath: imageSavePath,
    upscaleFactor: upscaleFactor.value,
  });
  isProcessing.value = false;
  alert(output);
}
</script>

<style scoped lang="scss">
.blocked-cursor {
  cursor: not-allowed;
}
.loading-gif {
  filter: blur(0.5px);
  margin-left: -80px;
  margin-top: 160px;
  position: fixed;
}
.upscale-options {
  text-align: left;
  align-items: flex-start;
  margin-bottom: 10px;
}
.image-src {
  border-radius: 24px;
  border: 3px solid rgba($color: #ffffff, $alpha: 0.4);
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
