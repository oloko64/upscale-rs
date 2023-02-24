<template>
  <div>
    <v-expansion-panels>
      <v-expansion-panel title="Advanced options">
        <v-expansion-panel-text>
          <v-card
            class="mt-2"
            variant="outlined"
          >
            <v-card-title>Multi GPU options</v-card-title>
            <v-card-text>
              <v-text-field
                v-model.trim="advancedOptions.gpu_id"
                :rules="[rules.gpuId]"
                class="mt-2"
                label="gpu-id"
                density="compact"
                variant="outlined"
              />
              <v-text-field
                v-model.trim="advancedOptions.tile_size"
                label="tile-size" 
                density="compact"
                variant="outlined"
              />
              <v-text-field
                v-model.trim="advancedOptions.load_proc_save"
                label="load-proc-save" 
                density="compact"
                variant="outlined"
                hide-details
              />
            </v-card-text>
          </v-card>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
  </div>
</template>

<script setup lang="ts">
import { AdvancedOptionsType } from "@/types/advancedOptions";
import { ref, watch } from "vue";

const emit = defineEmits(["advanced-options"]);

const rules = {
    gpuId: (val: string) => {
        const pattern = new RegExp(/^(\d+(,\d+)*)?$/);
        return pattern.test(val) || "gpu-id must be empty, a integer or a list of integers separated by commas";
    },
}

const advancedOptions = ref({
    gpu_id: "",
    tile_size: "",
    load_proc_save: "",
} as AdvancedOptionsType);

watch(advancedOptions, (newVal: AdvancedOptionsType) => {
    emit("advanced-options", newVal);
}, { deep: true });

</script>