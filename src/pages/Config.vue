<template>
  <div class="container">
    <h2>Options</h2>
    <v-switch
      v-model="options['application-logs']"
      inset
      class="ml-5"
      label="Application logs"
    ></v-switch>
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { watch, ref, onMounted } from "vue";

interface Configuration {
  ["application-logs"]: boolean;
}

const options = ref<Configuration>({
  ["application-logs"]: false,
});

onMounted(async () => {
  const config = await invoke<Configuration>("load_configuration");
  options.value = config;
});

watch(options.value, async () => {
  await invoke("write_configuration", { config: options.value });
});
</script>
