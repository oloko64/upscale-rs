<template>
  <div class="container">
    <h2>Options</h2>
    <v-switch v-model="options['application-logs']" hide-details inset class="ml-5" label="Save application logs"
      @change="write_configuration()" />
    <v-divider class="ml-5 mr-5 mb-5 mt-2" />
    <v-select class="select-fields ml-5" label="Default Upscale Type" v-model="options['default-upscale-type']"
      variant="solo" :items="optionsUpscaleType" item-title="text" item-value="value" @change="write_configuration()" />
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";

const optionsUpscaleType = [
  {
    text: 'General Image',
    value: 'general',
  },
  {
    text: 'Digital Image',
    value: 'digital',
  },
];

interface Configuration {
  ["application-logs"]: boolean;
  ["default-upscale-type"]: string;
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
</style>
