<template>
  <div>
    <v-dialog
      v-model="dialog"
      width="650"
    >
      <v-card
        class="mt-2"
      >
        <v-card-title>
          <div class="d-flex justify-space-between">
            Multi GPU options
            <v-icon @click="openGpuDocumentation">
              {{ mdiTooltipQuestionOutline }}
            </v-icon>
          </div>
        </v-card-title>
        <v-card-text>
          <v-text-field
            v-model.trim="advancedOptions.gpu_id"
            :rules="[rules.gpuId]"
            maxlength="20"
            class="mt-2"
            label="gpu-id"
            placeholder="gpu device to use (default=auto) can be 0,1,2 for multi-gpu"
            density="compact"
            variant="outlined"
          />
          <v-text-field
            v-model.trim="advancedOptions.tile_size"
            maxlength="20"
            label="tile-size"
            placeholder="tile size (>=32/0=auto, default=0) can be 0,0,0 for multi-gpu"
            density="compact"
            variant="outlined"
          />
          <v-text-field
            v-model.trim="advancedOptions.load_proc_save"
            maxlength="20"
            label="load/proc/save" 
            placeholder="thread count for load/proc/save (default=1:2:2) can be 1:2,2,2:2 for multi-gpu"
            density="compact"
            variant="outlined"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn
            text
            variant="outlined"
            height="30"
            @click="dialog = false"
          >
            Close
          </v-btn>
          <v-btn
            variant="outlined"
            height="30"
            @click="clearAdvancedOptions()"
          >
            Clear
          </v-btn>
        </v-card-actions>
      </v-card>
      <template #activator="{ props }">
        <v-btn
          v-bind="props"
          rounded="16"
          variant="text"
          height="30"
          width="100%"
          :disabled="defined_props.disabled"
          flat
        >
          Advanced options
        </v-btn>
      </template>
    </v-dialog>
    <div v-if="!isAdvancedOptionsEmpty">
      <p>Active advanced options:</p>
      <p v-if="advancedOptions.gpu_id">
        <strong>gpu-id: </strong>{{ advancedOptions.gpu_id }}
      </p>
      <p v-if="advancedOptions.tile_size">
        <strong>tile-size: </strong>{{ advancedOptions.tile_size }}
      </p>
      <p v-if="advancedOptions.load_proc_save">
        <strong>load/proc/save: </strong>{{ advancedOptions.load_proc_save }}
      </p>
      <div class="d-flex justify-end">
        <v-btn
          class="mt-2"
          variant="outlined"
          height="30"
          text
          @click="clearAdvancedOptions()"
        >
          Clear
        </v-btn>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AdvancedOptionsType } from "@/types/advancedOptions";
import { ref, watch, computed } from "vue";
import { mdiTooltipQuestionOutline } from "@mdi/js";
import { open } from '@tauri-apps/api/shell';
import { sendTauriNotification } from "@/helpers/tauriNotification";

const emit = defineEmits(["advanced-options"]);
const defined_props = defineProps<{
  disabled: boolean;
}>();

const rules = {
  gpuId: (val: string) => {
    const pattern = new RegExp(/^(\d+(,\d+)*)?$/);
    return pattern.test(val) || "gpu-id must be empty, a integer or a list of integers separated by commas";
  },
}

const dialog = ref(false);
const advancedOptions = ref({
    gpu_id: "",
    tile_size: "",
    load_proc_save: "",
} as AdvancedOptionsType);

const isAdvancedOptionsEmpty = computed(() => {
    return advancedOptions.value.gpu_id === "" && advancedOptions.value.tile_size === "" && advancedOptions.value.load_proc_save === "";
});

function clearAdvancedOptions() {
    advancedOptions.value = {
        gpu_id: "",
        tile_size: "",
        load_proc_save: "",
    };
}

async function openGpuDocumentation() {
  try {
    await open("https://github.com/xinntao/Real-ESRGAN#usage-of-portable-executable-files");
  } catch (err) {
    sendTauriNotification(
      "Error opening documentation",
      "Please open the documentation manually at https://github.com/xinntao/Real-ESRGAN#usage-of-portable-executable-files"
    );
  }
}

watch(advancedOptions, (newVal: AdvancedOptionsType) => {
    emit("advanced-options", newVal);
}, { deep: true });

</script>
