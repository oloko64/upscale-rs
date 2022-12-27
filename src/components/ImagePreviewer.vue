<template>
    <div>
        <viewer :images="images" />
        <div :class="{ 'preview-area': !isProcessing }" @click="showImagePreviewer">
            <v-img class="image-src" :src="imageSrc" width="500" height="500" aspect-ratio="1" cover />
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

.image-src {
    border-radius: 24px;
    border: 2px solid rgba($color: #969696, $alpha: 0.4);
}

.preview-area {
    cursor: pointer;
}

.image-src:hover {
    filter: brightness(0.8);
    border: 2px solid rgba($color: #969696, $alpha: 0.8);
}
</style>
