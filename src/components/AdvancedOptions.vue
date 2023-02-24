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
            v-model.trim="configOptions['advanced-options']['gpu-id']"
            :rules="[rules.gpuId]"
            maxlength="20"
            class="mt-2"
            label="gpu-id"
            placeholder="gpu device to use (default=auto) can be 0,1,2 for multi-gpu"
            density="compact"
            variant="outlined"
          />
          <v-text-field
            v-model.trim="configOptions['advanced-options']['tile-size']"
            maxlength="20"
            label="tile-size"
            placeholder="tile size (>=32/0=auto, default=0) can be 0,0,0 for multi-gpu"
            density="compact"
            variant="outlined"
          />
          <v-text-field
            v-model.trim="configOptions['advanced-options']['load-proc-save']"
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
            @click="writeConfiguration()"
          >
            Save
          </v-btn>
          <v-btn
            variant="outlined"
            height="30"
            @click="clearAdvancedOptions()"
          >
            Clear
          </v-btn>
          <v-btn
            text
            variant="outlined"
            height="30"
            @click="dialog = false"
          >
            Close
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
      <p v-if="configOptions['advanced-options']?.['gpu-id']">
        <strong>gpu-id: </strong>{{ configOptions['advanced-options']?.['gpu-id'] }}
      </p>
      <p v-if="configOptions['advanced-options']?.['tile-size']">
        <strong>tile-size: </strong>{{ configOptions['advanced-options']?.['tile-size'] }}
      </p>
      <p v-if="configOptions['advanced-options']?.['load-proc-save']">
        <strong>load/proc/save: </strong>{{ configOptions['advanced-options']?.['load-proc-save'] }}
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
import { ref, watch, computed, onMounted } from "vue";
import { mdiTooltipQuestionOutline } from "@mdi/js";
import { open } from '@tauri-apps/api/shell';
import { sendTauriNotification } from "@/helpers/tauriNotification";
import { Configuration } from "@/types/configuration";
import { invoke } from "@tauri-apps/api/tauri";

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

const isAdvancedOptionsEmpty = computed(() => {
    return !configOptions.value["advanced-options"]?.['gpu-id'] 
    && !configOptions.value["advanced-options"]?.["tile-size"] 
    && !configOptions.value["advanced-options"]?.["load-proc-save"];
});

const configOptions = ref({} as Configuration);

onMounted(async () => {
  try {
    const config = await invoke<Configuration>("load_configuration");
      configOptions.value = config;
  } catch (error) {
    sendTauriNotification(
      "Error",
      "Error loading configuration file"
      );
  }
});

async function writeConfiguration() {
  try {
    await invoke("write_configuration", { config: configOptions.value });
  } catch (error) {
    sendTauriNotification(
      "Error",
      "Error writing configuration file"
      );
  }
}

function clearAdvancedOptions() {
  configOptions.value["advanced-options"] = {
    ["gpu-id"]: "",
    ["tile-size"]: "",
    ["load-proc-save"]: "",
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

watch(configOptions, (newVal: Configuration) => {
    emit("advanced-options", newVal["advanced-options"]);
}, { deep: true });

</script>
