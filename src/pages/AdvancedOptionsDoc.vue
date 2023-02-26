<template>
  <div class="container">
    <h2>
      Advanced Options Documentation
    </h2>
    <div class="text-start mx-5 mt-8">
      <v-expansion-panels variant="accordion">
        <v-expansion-panel>
          <v-expansion-panel-title>
            <h3 class="expansion-panel-title">
              TTA
            </h3>
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            <code>
              <p>TTA (Test-Time Augmentation) averages the upscaling results of the following 8 augmented inputs.</p>
              <p>TTA mode able to reduce several type of artifacts but it's 8x slower than non TTA mode.</p>
            </code>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-title>
            <h3 class="expansion-panel-title">
              gpu-id
            </h3>
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            <code>
              <p>gpu device to use (default=auto) can be 0,1,2 for multi-gpu.</p>
            </code>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-title>
            <h3 class="expansion-panel-title">
              tile-size
            </h3>
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            <code>
              <p>Tile size (>=32/0=auto, default=0) can be 0,0,0 for multi-gpu.</p>
              <p>Use smaller value to reduce GPU memory usage, default selects automatically.</p>
            </code>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-title>
            <h3 class="expansion-panel-title">
              load:proc:save
            </h3>
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            <code>
              <p>Thread count for load/proc/save (default=1:2:2) can be 1:2,2,2:2 for multi-gpu.</p>
              <p>Thread count for the three stages (image decoding + model upscaling + image encoding), using larger values may increase GPU usage and consume more GPU memory.
              </p>
              <p>You can tune this configuration with "4:4:4" for many small-size images, and "2:2:2" for large-size images. The default setting usually works fine for most situations. If you find that your GPU is hungry, try increasing thread count to achieve faster processing.</p>
            </code>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
    </div>
    <div class="d-flex justify-end">
      <a
        class="external-docs mr-5 mt-5"
        @click="openGpuExternalDocumentation"
      >External Documentation</a>
    </div>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/shell';

function openGpuExternalDocumentation() {
  open("https://github.com/xinntao/Real-ESRGAN#usage-of-portable-executable-files");
}
</script>

<style scoped lang="scss">
.expansion-panel-title {
    width: 100%;
}

.external-docs {
    text-decoration: underline;
    cursor: pointer;
}
</style>
