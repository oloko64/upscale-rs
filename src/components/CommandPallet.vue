<template>
  <div class="card">
    <button type="button" @click="openImage()">Open Image</button>
    <button type="button" @click="upscaleSingleImage()">
      Upscale Selected Image
    </button>
    <h3>{{ imagePath }}</h3>
    <img :src="imageBlob" width="500" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";

const imagePath = ref("");
const imageBlob = ref("");

async function openImage() {
  // Open a selection dialog for image files
  const selected = await open({
    filters: [
      {
        name: "",
        extensions: ["png", "jpeg"],
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
  const imageSavePath = await save();
  invoke("upscale_single_image", {
    path: imagePath.value,
    savePath: imageSavePath,
  });
}
</script>
