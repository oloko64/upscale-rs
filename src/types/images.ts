import { Ref } from "vue";

export interface ImagePathsDisplay {
    path: string;
    isReady: boolean;
    progressPercentageMulti: Ref<string>;
}