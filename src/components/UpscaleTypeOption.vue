<template>
  <div class="upscale-types">
    <v-select :disabled="props.disabled" label="Upscale Type" v-model="selectType" variant="solo" density="comfortable"
      :items="upscaleTypeOptions" item-title="text" item-value="value" hide-details />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const upscaleTypeOptions = [
  {
    text: "General Image",
    value: "general",
  },
  {
    text: "Digital Image",
    value: "digital",
  },
];

const props = defineProps<{
  disabled: boolean;
}>();

// The upscale type. Default is `general`.
const selectType = ref("general");

const emit = defineEmits(["upscale-type-changed"]);

onMounted(async () => {
  try {
    const config = await invoke<{ ["default-upscale-type"]: string }>(
      "load_configuration"
    );
    selectType.value = config["default-upscale-type"];
  } catch (error: any) {
    await invoke("write_log", { message: error.toString() });
    alert(error);
  }
});

// Watch for the select between `general` and `digital` type and sends selected type to the parent component.
watch(selectType, (value) => {
  emit("upscale-type-changed", value);
});
</script>

<style scoped lang="scss">
.upscale-types {
  display: inline-block;
}
</style>
