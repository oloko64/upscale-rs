<template>
  <div class="container">
    <h2>Options</h2>
    <v-switch
      v-model="options['application-logs']"
      hide-details
      inset
      class="ml-5"
      label="Save application logs"
    ></v-switch>
    <v-divider class="ml-5 mb-5 mt-2 options-spacer" />
    <v-select
      class="select-fields ml-5"
      label="Default Upscale Type"
      v-model="options['default-upscale-type']"
      variant="solo"
      :items="[
        {
          text: 'General Image',
          value: 'general',
        },
        {
          text: 'Digital Image',
          value: 'digital',
        },
      ]"
      item-title="text"
      item-value="value"
    ></v-select>
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { watch, ref, onMounted } from "vue";

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

watch(
  () => options.value,
  async (updatedValue) => {
    try {
      await invoke("write_configuration", { config: updatedValue });
    } catch (error) {
      alert(error);
    }
  },
  { deep: true }
);
</script>

<style scoped lang="scss">
.select-fields {
  width: 300px;
}

.options-spacer {
  width: 350px;
}
</style>
