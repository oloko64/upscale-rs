<template>
  <div class="upscale-types">
    <select class="select" name="upscale-option" id="upscale-option" @change="updateUpscaleType">
      <option class="select-option" value="general">General Image</option>
      <option class="select-option" value="digital">Digital Art</option>
    </select>
  </div>
</template>

<script setup lang="ts">
const emit = defineEmits(["upscale-type-changed"]);

function updateUpscaleType(value: any) {
  emit("upscale-type-changed", value.target.value);
}
</script>

<style scoped lang="scss">
.select {
  width: 100%;
  min-width: 15ch;
  max-width: 30ch;
  border-radius: 0.50em;
  padding: 0.25em 0.5em;
  font-size: 1rem;
  cursor: pointer;
  line-height: 1.1;
  background-color: #fff;
  background-image: linear-gradient(to top, #f9f9f9, #fff 33%);
}
.select-option {
  cursor: pointer;
  display: inline-block;
  position: relative;
  font-size: 16px;
  width: 100%;
  height: 40px;
  color: rgb(254, 254, 254);
}

.upscale-types {
  display: inline-block;
  margin-top: 5px;
  margin-bottom: 25px;
}
</style>
