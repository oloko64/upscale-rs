<template>
  <div class="container">
    <h2>Options</h2>
    <v-switch
      v-model="options['application-logs']"
      hide-details
      inset
      class="ml-5"
      label="Save application logs"
      @update:modelValue="write_configuration()"
    />
    <v-divider class="ml-5 mr-5 mb-5 mt-2" />
    <v-select
      class="select-fields ml-5"
      label="Default Upscale Type"
      v-model="options['default-upscale-type']"
      variant="solo"
      density="comfortable"
      :items="optionsUpscaleType"
      item-title="text"
      item-value="value"
      hide-details
      @update:modelValue="write_configuration()"
    />
    <v-divider class="ml-5 mr-5 mb-5 mt-6" />
    <div class="max-size-option">
      <p class="pl-5 ma-0 mb-3">
        Set a size limit of upscaled images in MB to show in the image previewer
        (After 25Mb it can become very laggy)
      </p>
      <v-select
        class="select-fields ml-5"
        label="Size in MB"
        v-model="options['max-preview-upscale-size']"
        variant="solo"
        density="comfortable"
        :items="optionsPreviewMaxSize"
        hide-details
        @update:modelValue="write_configuration()"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";

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

interface Configuration {
  ["application-logs"]: boolean;
  ["default-upscale-type"]: string;
  ["max-preview-upscale-size"]: number;
}

const options = ref({} as Configuration);

onMounted(async () => {
  try {
    const config = await invoke<Configuration>("load_configuration");
    options.value = config;
  } catch (error) {
    alert(error);
  }
});

function write_configuration() {
  invoke("write_configuration", { config: options.value });
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
