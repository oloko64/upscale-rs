<template>
  <router-view></router-view>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted } from "vue";
import { useTheme } from "vuetify";
import { platform } from "@tauri-apps/api/os";
import { appWindow } from "@tauri-apps/api/window";
const theme = useTheme();

onMounted(async () => {
  const appTheme = await appWindow.theme();
  const platformName = await platform();

  if (appTheme && platformName === 'win32') {
    theme.global.name.value = appTheme;
  }
});
</script>
