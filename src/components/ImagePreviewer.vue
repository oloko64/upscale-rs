<template>
    <div>
        <viewer :images="images" />
        <div class="show-image-preview">
            <v-btn elevation="0" @click="showImagePreviewer">Click to preview images</v-btn>
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
        }
    },
    methods: {
        showImagePreviewer() {
            this.$viewerApi({
                images: this.images as string[],
                options: {
                    inline: true,
                    button: false,
                    toolbar: false,
                    title: false,
                    tooltip: false,
                    rotatable: false,
                    scalable: false,
                    fullscreen: false,
                    keyboard: false,
                },
            })
        },
    },
})
</script>
<style scoped lang="scss">
.show-image-preview {
    z-index: 1;
    top: 500px;
    left: 500px;
    position: fixed;
}
</style>