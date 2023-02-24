<template>
  <div class="container">
    <h2>Options</h2>
    <v-switch
      v-model="options['application-logs']"
      hide-details
      inset
      class="ml-5"
      label="Save application logs"
      @update:model-value="writeConfiguration()"
    />
    <v-divider class="ml-5 mr-5 mb-5 mt-2" />
    <v-select
      v-model="options['default-upscale-type']"
      class="select-fields ml-5"
      label="Default Upscale Type"
      variant="solo"
      density="comfortable"
      :items="optionsUpscaleType"
      item-title="text"
      item-value="value"
      hide-details
      @update:model-value="writeConfiguration()"
    />
    <v-divider class="ml-5 mr-5 mb-5 mt-6" />
    <div class="max-size-option">
      <p class="pl-5 ma-0 mb-3">
        Set a size limit of images to be shown in the image previewer after upscale
        (After 25Mb it can become very laggy)
      </p>
      <v-select
        v-model="options['max-preview-upscale-size']"
        class="select-fields ml-5"
        label="Size in MB"
        variant="solo"
        density="comfortable"
        :items="optionsPreviewMaxSize"
        hide-details
        @update:model-value="writeConfiguration()"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";
import { Configuration } from "@/types/configuration";
import { sendTauriNotification } from "@/helpers/tauriNotification";

const optionsPreviewMaxSize = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];

const optionsUpscaleType = [
  {
    text: "General Image",
    value: "general",
  },
  {
    text: "Digital Image",
    value: "digital",
  },
];

const options = ref({} as Configuration);

onMounted(async () => {
  try {
    const config = await invoke<Configuration>("load_configuration");
    options.value = config;
  } catch (error) {
    sendTauriNotification(
      "Error",
      "Error loading configuration file",
    );
  }
});

async function writeConfiguration() {
  try {
    await invoke("write_configuration", { config: options.value });
  } catch (error) {
    sendTauriNotification(
      "Error",
      "Error writing configuration file",
    );
  }
}
</script>

<style scoped lang="scss">
.select-fields {
  width: 300px;
}

.max-size-option {
  display: block;
  text-align: left;
}
</style>
