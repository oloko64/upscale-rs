<template>
  <div class="upscale-types">
    <v-select
      label="Upscale Type"
      v-model="selectType"
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
import { ref, watch } from "vue";

const selectType = ref("general");

const emit = defineEmits(["upscale-type-changed"]);

watch(selectType, (value) => {
  emit("upscale-type-changed", value);
});
</script>

<style scoped lang="scss">
.upscale-types {
  display: inline-block;
  margin-top: 5px;
  margin-bottom: 25px;
}
</style>
