# Upscale-rs

![upscale-rs-horizontal](https://user-images.githubusercontent.com/49915167/203207059-1ed3fd08-6cff-4068-a9d7-5b2559d1fb94.png)

[![Upscale-rs Workflow](https://github.com/OLoKo64/upscale-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/OLoKo64/upscale-rs/actions/workflows/rust.yml)

A GUI image upscaler making use of [Tauri](https://tauri.app/) and [Vue3](https://vuejs.org/) with [Vuetify](https://next.vuetifyjs.com/en/) as it's component library.

Using the [Real-ESRGAN](https://github.com/xinntao/Real-ESRGAN) model under the hood. **It requires a GPU to run**.

# Features

- 4x upscale of images with 2 configurations general photo and anime.
- Single and multiple images upscale.
- Real time percentage progress.
- Support for `png`, `jpg` and `webp` images.
- Option to use TTA (Test Time Augmentation) for reduced artifacts.
- Ability to select which GPU to use as well as `tile-size` and `load:proc:save` so you can fine tune the performance.
- Inside app image previewer.
- Drag and drop of images for easy of use.
- No internet needed, can be used offline.
- Build in logging for better debugging (optional and off by default).
- Light and Dark modes, based on your app configuration (Windows only feature, Linux uses Dark mode).

# Interface

![image](https://user-images.githubusercontent.com/49915167/222283230-22257c4f-eae6-40b8-90c1-246a89124414.png)

# Upscaling Sample

![before-after-ia](https://user-images.githubusercontent.com/49915167/203209186-4fc7470a-acd4-4ad5-bab5-ef1df76496b1.jpg)
