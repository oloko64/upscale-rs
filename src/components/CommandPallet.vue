<template>
  <div class="outer-box">
    <div class="options-column">
      <v-btn
        size="large"
        rounded="lg"
        :prepend-icon="mdiFileImage"
        :disabled="isProcessing"
        elevation="0"
        @click="openImage"
      >
        Select Image
      </v-btn>
      <UpscaleTypeOption @upscale-type-changed="setUpscaleType" />
      <!-- Scale factor seems not to be working -->
      <!-- <UpscaleFactorOptions @upscale-factor-changed="updateUpscaleFactor" /> -->
      <v-btn
        size="large"
        rounded="lg"
        class="mb-2"
        :disabled="isProcessing || !imagePath"
        elevation="0"
        @click="upscaleSingleImage"
      >
        Upscale Selected Image
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
    <div class="image-area mt-5">
      <h4 class="mb-2">{{ imagePath }}</h4>
      <v-progress-circular
        class="loading-gif"
        color="primary"
        indeterminate
        :size="128"
        :width="12"
        v-if="isProcessing"
      ></v-progress-circular>
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
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";
import UpscaleFactorOptions from "./UpscaleFactorOptions.vue";
import UpscaleTypeOption from "./UpscaleTypeOption.vue";
import { mdiFileImage } from "@mdi/js";

const isProcessing = ref(false);
const imagePath = ref("");
const imageBlob = ref("");
const upscaleFactor: Ref<"2" | "3" | "4"> = ref("4");
const upscaleType: Ref<"general" | "digital"> = ref("general");

function setUpscaleType(value: any) {
  upscaleType.value = value;
}

function clearSelectedImage() {
  imagePath.value = "";
  imageBlob.value = "";
}

// function updateUpscaleFactor(value: any) {
//   upscaleFactor.value = value.target.value;
// }

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
    try {
      const imageBytes = await invoke("read_image_base64", { path: selected });
      imageBlob.value = `data:image/png;base64,${imageBytes}`;
    } catch (err) {
      alert(err);
    }
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
  text-align: center;
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
