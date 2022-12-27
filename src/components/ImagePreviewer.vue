<template>
    <div>
        <viewer :images="images" />
        <div :class="{ 'preview-area': !isProcessing }" @click="showImagePreviewer">
            <v-img :class="{ 'image-src-on-hover': !isProcessing }" class="image-src" :src="imageSrc" width="500" height="500" aspect-ratio="1" cover >
                <div class="preview-text-back px-3 py-2">
                    <p class="ma-0">Click to preview</p>
                </div>
            </v-img>
        </div>
    </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue'
export default defineComponent({
    props: {
        images: {
            type: Array,
            default: () => []
        },
        isProcessing: {
            type: Boolean,
            default: false
        }
    },
    computed: {
        imageSrc() {
            return this.images[0] as string
        }
    },
    methods: {
        showImagePreviewer() {
            if (this.isProcessing) {
                return
            }
            this.$viewerApi({
                images: this.images as string[],
                options: {
                    inline: true,
                    toolbar: false,
                    title: false,
                    tooltip: false,
                    rotatable: false,
                    scalable: false,
                    fullscreen: false,
                },
            })
        },
    },
})
</script>
<style scoped lang="scss">
.show-image-preview {
    top: 500px;
    left: 500px;
    position: fixed;
}

.preview-text-back {
    z-index: 1;
    position: absolute;
    top: 440px;
    left: 180px;
    color: #e7e7e7;
    background-color: rgba(34, 34, 34, 0.7);
    border-radius: 24px;
    font-size: 20px;
    font-weight: 500;
}

.image-src {
    border-radius: 24px;
    border: 2px solid rgba($color: #969696, $alpha: 0.4);
}

.preview-area {
    cursor: pointer;
}

.image-src-on-hover:hover {
    filter: brightness(0.7);
    border: 2px solid rgba($color: #969696, $alpha: 0.8);
}
</style>
