<template>
    <div class="outer-box">
        <div class="options-column">
            <img
                class="mb-3 about-logo-redirect"
                :src="HorizontalLogo"
                width="200"
                @click="openAboutPage"
            />
            <v-btn
                class="mt-6"
                size="large"
                rounded="lg"
                :prepend-icon="mdiFileImage"
                :disabled="isProcessing"
                elevation="0"
                @click="openImage"
            >
                Select Images
            </v-btn>
            <UpscaleTypeOption
                :disabled="isProcessing"
                class="mt-2 mb-2"
                @upscale-type-changed="setUpscaleType"
            />
            <AdvancedOptions
                :disabled="isProcessing"
                class="mt-3"
                @advanced-options="updateAdvancedOptions"
            />
            <v-divider class="mb-6 mt-3" />
            <!-- Scale factor seems not to be working -->
            <!-- <UpscaleFactorOptions @upscale-factor-changed="updateUpscaleFactor" /> -->
            <v-btn
                size="large"
                rounded="lg"
                class="mb-2"
                :disabled="isReadyToUpscale"
                elevation="0"
                width="310"
                @click="startProcessing"
            >
                {{
                    isMultipleFiles
                        ? "Upscale Selected Images"
                        : "Upscale Selected Image"
                }}
            </v-btn>
            <v-btn
                size="large"
                rounded="lg"
                :disabled="isProcessing"
                elevation="0"
                @click="clearSelectedImage"
            >
                Clear
            </v-btn>
            <div class="d-flex">
                <v-btn
                    elevation="0"
                    class="config-options-button"
                    size="32"
                    :disabled="isProcessing"
                    :icon="mdiMenu"
                    @click="openConfig"
                />
            </div>
        </div>
        <div
            class="image-area mt-5"
            :class="{ 'text-center': !isMultipleFiles }"
        >
            <h5 v-if="imagePath" class="mb-2 path-text">
                {{ imagePath }}
            </h5>
            <h5
                v-for="imgPath in imagePaths"
                :key="imgPath.path"
                class="mb-2 path-text"
            >
                <v-progress-circular
                    v-if="!imgPath.isReady"
                    v-show="showMultipleFilesProcessingIcon"
                    indeterminate
                    color="#ff7a00"
                    size="16"
                />
                <span
                    v-if="!imgPath.isReady"
                    v-show="showMultipleFilesProcessingIcon"
                >
                    - {{ imgPath.progressPercentageMulti }} |</span
                >
                <v-icon
                    v-if="imgPath.isReady"
                    v-show="showMultipleFilesProcessingIcon"
                    size="16"
                    :icon="mdiImageCheck"
                />
                <span class="ml-2">{{ imgPath.path }}</span>
                <v-divider />
            </h5>
            <div v-if="isProcessing && !isMultipleFiles">
                <span class="loading-percentage-text">{{
                    progressPercentage
                }}</span>
                <v-progress-circular
                    class="loading-gif"
                    color="#ff7a00"
                    indeterminate
                    :size="128"
                    :width="12"
                />
            </div>
            <div
                v-if="!imageBlob && !imagePaths.length"
                class="file-drop-area mt-8"
                @click="openImage"
            >
                Click to select images or drop them here
            </div>
            <ImagePreviewer
                v-if="imageBlob"
                :images="[upscaledImageBlob, imageBlob]"
                :is-processing="isProcessing"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import HorizontalLogo from "@/assets/upscale-rs-horizontal.png";
import UpscaleTypeOption from "@/components/UpscaleTypeOption.vue";
import ImagePreviewer from "@/components/ImagePreviewer.vue";
import { AdvancedOptionsType } from "@/types/advancedOptions";
import AdvancedOptions from "@/components/AdvancedOptions.vue";
import { mdiFileImage, mdiImageCheck, mdiMenu } from "@mdi/js";
import { invoke } from "@tauri-apps/api/tauri";
import { loadImage } from "@/helpers/loadImageBase64";
import { sendTauriNotification } from "@/helpers/tauriNotification";
import { open, save } from "@tauri-apps/api/dialog";
import { WebviewWindow } from "@tauri-apps/api/window";
import { Configuration } from "@/types/configuration";
import { ImagePathsDisplay } from "@/types/images";
import { appWindow } from "@tauri-apps/api/window";

const DEFAULT_PERCENTAGE = "0.00%";

type UpscaleType = "general" | "digital";
type UpscaleFactor = "2" | "3" | "4";

const isProcessing = ref(false);
const imagePath = ref("");
const imagePaths: Ref<ImagePathsDisplay[]> = ref([]);
const imageBlob = ref("");
const upscaledImageBlob = ref("");
const upscaleFactor: Ref<UpscaleFactor> = ref("4");
const upscaleType: Ref<UpscaleType> = ref("general");
const isMultipleFiles = ref(false);
const showMultipleFilesProcessingIcon = ref(false);
const progressPercentage = ref(DEFAULT_PERCENTAGE);

const advancedOptions = ref({
    ["gpu-id"]: "",
    ["tile-size"]: "",
    ["load-proc-save"]: "",
} as AdvancedOptionsType);

// Computes if the user is ready to upscale the image. Used the simplify the DOM code.
const isReadyToUpscale = computed(() => {
    return !(
        (imagePath.value || imagePaths.value.length > 0) &&
        !isProcessing.value
    );
});

appWindow.listen(
    "UPSCALE-PERCENTAGE",
    ({ event, payload }: { event: any; payload: string }) => {
        progressPercentage.value = payload;
    }
);

/**
 * Listens for file drops on the window and decides if is a single or multiple file upload.
 */
appWindow.listen(
    "tauri://file-drop",
    async ({ event, payload }: { event: any; payload: string[] }) => {
        const files = payload;
        if (!files.length || isProcessing.value) {
            return;
        }
        clearSelectedImage();
        const validExtensions = (path: string) =>
            path.endsWith(".png") ||
            path.endsWith(".jpg") ||
            path.endsWith(".jpeg") ||
            path.endsWith(".webp");
        if (files.length > 1) {
            showMultipleFilesProcessingIcon.value = false;
            isMultipleFiles.value = true;
            imagePaths.value = files
                .map((file) => {
                    return {
                        path: file,
                        isReady: false,
                        progressPercentageMulti: ref(DEFAULT_PERCENTAGE),
                    };
                })
                .filter((file) => validExtensions(file.path));
        } else {
            const image = files[0];
            isMultipleFiles.value = false;
            if (!validExtensions(image)) {
                alert("Please select a valid image file.");
                return;
            }
            try {
                imageBlob.value = await loadImage(image);
                imagePath.value = image;
            } catch (err: any) {
                await invoke("write_log", {
                    message: `Error reading image: ${err}`,
                });
            }
        }
    }
);

function updateAdvancedOptions(options: AdvancedOptionsType) {
    advancedOptions.value = options;
}

function openAboutPage() {
    // https://tauri.app/v1/guides/features/multiwindow#create-a-window-in-javascript
    const webview = new WebviewWindow("about-page", {
        height: 400,
        width: 500,
        title: "About",
        url: "/about",
    });
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once("tauri://created", function () {
        // webview window successfully created
    });
    webview.once("tauri://error", function (err) {
        console.error(err);
        // an error happened creating the webview window
    });
}

/**
 * Sets the upscale type.
 *
 * @param value - The upscale type. Available values are `general` and `digital`.
 */
function setUpscaleType(value: UpscaleType) {
    upscaleType.value = value;
}

function openConfig() {
    // https://tauri.app/v1/guides/features/multiwindow#create-a-window-in-javascript
    const webview = new WebviewWindow("config-page", {
        height: 400,
        width: 850,
        title: "Config",
        url: "/config",
    });
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once("tauri://created", function () {
        // webview window successfully created
    });
    webview.once("tauri://error", function (err) {
        console.error(err);
        // an error happened creating the webview window
    });
}

/**
 * Clears the selected image and some other variables.
 */
function clearSelectedImage() {
    imagePath.value = "";
    imagePaths.value = [];
    imageBlob.value = "";
    upscaledImageBlob.value = "";
    showMultipleFilesProcessingIcon.value = false;
    isMultipleFiles.value = false;
}

/**
 * Sets the upscale factor. Currently it's not working.
 */
// function updateUpscaleFactor(value: UpscaleFactor) {
//   upscaleFactor.value = value;
// }

/**
 * Opens the image file dialog from Tauri.
 *
 * It is used in the single and multiple file selector.
 */
async function openImage() {
    // Open a selection dialog for image files
    const selected = await open({
        multiple: true,
        filters: [
            {
                name: "",
                extensions: ["png", "jpeg", "jpg"],
            },
        ],
    });
    if (Array.isArray(selected) && selected.length > 1) {
        clearSelectedImage();
        isMultipleFiles.value = true;
        showMultipleFilesProcessingIcon.value = false;
        imagePaths.value = selected.map((path) => {
            return {
                path,
                isReady: false,
                progressPercentageMulti: ref(DEFAULT_PERCENTAGE),
            };
        });
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        clearSelectedImage();
        isMultipleFiles.value = false;
        imagePath.value = selected[0];
        try {
            imageBlob.value = await loadImage(imagePath.value);
        } catch (err: any) {
            await invoke("write_log", { message: err.toString() });
            alert(err);
        }
    }
}

/**
 * Runs the correct single or multiple file processing function.
 *
 * It is used to control the code flow.
 */
function startProcessing() {
    progressPercentage.value = DEFAULT_PERCENTAGE;
    if (isMultipleFiles.value) {
        upscaleMultipleImages();
    } else {
        upscaleSingleImage();
    }
}

/**
 * Upscales multiple images function.
 *
 * It will ask the user to select a folder to save the upscaled images.
 *
 * It will update the `isReady` property of the `imagePaths` array to true when the image is ready.
 */
async function upscaleMultipleImages() {
    const outputFolder = await open({
        directory: true,
    });
    if (outputFolder === null) {
        return;
    }
    isProcessing.value = true;
    showMultipleFilesProcessingIcon.value = true;
    try {
        for (let i = 0; i < imagePaths.value.length; i++) {
            let outputFile: string = await invoke("replace_file_suffix", {
                path: imagePaths.value[i].path,
            });

            outputFile = `${outputFolder}/${outputFile.split("/").pop()}`;

            // Copy the progress percentage by reference for the current processing image
            imagePaths.value[i].progressPercentageMulti = progressPercentage;
            await invoke("upscale_single_image", {
                path: imagePaths.value[i].path,
                savePath: outputFile,
                upscaleFactor: upscaleFactor.value,
                upscaleType: upscaleType.value,
                advancedOptions: advancedOptions.value,
            });
            imagePaths.value[i].isReady = true;
            progressPercentage.value = DEFAULT_PERCENTAGE;
        }
        sendTauriNotification(
            "Upscale-rs",
            "All images have been successfully upscaled!"
        );
    } catch (err: any) {
        showMultipleFilesProcessingIcon.value = false;
        await invoke("write_log", { message: err.toString() });
        alert(err);
    } finally {
        isProcessing.value = false;
    }
}

/**
 * Upscales a single image.
 *
 * It will ask the user to select a file name and location to save the upscaled image.
 *
 * After the image is upscaled, it will send a `alert` to the user.
 */
async function upscaleSingleImage() {
    if (imagePath.value === "") {
        alert("No image selected");
        return;
    }
    const imageSavePath = await save({
        defaultPath: await invoke("replace_file_suffix", {
            path: imagePath.value,
        }),
    });
    if (imageSavePath === null) {
        // user cancelled the selection
        return;
    }
    isProcessing.value = true;
    try {
        await invoke("upscale_single_image", {
            path: imagePath.value,
            savePath: imageSavePath,
            upscaleFactor: upscaleFactor.value,
            upscaleType: upscaleType.value,
            advancedOptions: advancedOptions.value,
        });
        sendTauriNotification(
            "Upscale-rs",
            "The image was successfully upscaled!"
        );

        // Load the upscaled image into the image previewer
        try {
            const config = await invoke<Configuration>("load_configuration");
            upscaledImageBlob.value = await loadImage(
                imageSavePath,
                config["max-preview-upscale-size"]
            );
        } catch (err: any) {
            await invoke("write_log", { message: err.toString() });
        }
    } catch (err: any) {
        await invoke("write_log", { message: err.toString() });
        alert(err);
    } finally {
        isProcessing.value = false;
    }
}
</script>

<style scoped lang="scss">
.loading-gif {
    z-index: 1;
    left: 535px;
    top: 235px;
    position: absolute;
}

.loading-percentage-text {
    background-color: rgba($color: #000000, $alpha: 0.5);
    border-radius: 12px;
    padding: 8px 4px 1px 4px;
    z-index: 1;
    font-size: 28px;
    top: 282px;
    left: 546px;
    position: absolute;
}

.image-area {
    min-width: 500px;
    min-height: 500px;
}

.path-text {
    font-size: 14px;
    font-weight: normal;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.config-options-button {
    position: fixed;
    top: 525px;
    left: 15px;
}

.file-drop-area {
    min-width: 500px;
    min-height: 500px;
    border: 2px dashed rgba($color: #969696, $alpha: 0.4);
    border-radius: 24px;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    cursor: pointer;
}

.outer-box {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width: 800px;
    height: 100%;
}

.about-logo-redirect {
    margin-left: 2px;
    margin-bottom: 0px !important;
    height: 30px;
    cursor: pointer;
}

.options-column {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    justify-content: center;
    width: 100%;
    height: 100%;
    padding: 20px;
    box-sizing: border-box;
}
</style>
