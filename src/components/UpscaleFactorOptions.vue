<template>
  <div class="upscale-options">
    <p style="margin-bottom: 2px">Select the upscale factor:</p>
    <input type="radio" id="2" name="fav_language" value="2" @change="updateUpscaleFactor" />
    <label style="margin-left: 5px" for="2">2x</label><br />
    <input type="radio" id="3" name="fav_language" value="3" @change="updateUpscaleFactor" />
    <label style="margin-left: 5px" for="3">3x</label><br />
    <input checked type="radio" id="4" name="fav_language" value="4" @change="updateUpscaleFactor" />
    <label style="margin-left: 5px" for="4">4x</label><br />
  </div>
</template>

<script setup lang="ts">
const emit = defineEmits(["upscale-factor-changed"]);

function updateUpscaleFactor(value: any) {
  emit("upscale-factor-changed", value.target.value);
}
</script>

<style scoped lang="scss">
.upscale-options {
  text-align: left;
  align-items: flex-start;
  margin-bottom: 10px;
}
</style>
