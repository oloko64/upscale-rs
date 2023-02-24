import { AdvancedOptionsType } from "@/types/advancedOptions";

export interface Configuration {
    ["application-logs"]: boolean;
    ["default-upscale-type"]: string;
    ["max-preview-upscale-size"]: number;
    ["advanced-options"]: AdvancedOptionsType;
}
