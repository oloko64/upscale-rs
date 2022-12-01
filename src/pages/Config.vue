<template>
  <div class="container">
    <h2>Options</h2>
    <v-switch
      v-model="options['upscale-logs']"
      inset
      class="ml-5"
      label="Save logs of the upscaling process"
    ></v-switch>
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { watch, ref, onMounted } from "vue";

interface Configuration {
  ["upscale-logs"]: boolean;
}

const options = ref<Configuration>({
  ["upscale-logs"]: false,
});

onMounted(async () => {
  const config = await invoke<Configuration>("load_configuration");
  options.value = config;
});

watch(
  () => options.value,
  async (updatedValue) => {
    await invoke("write_configuration", { config: updatedValue });
  },
  { deep: true }
);
</script>
