<script lang="ts" setup>
import Password from "primevue/password";
import { ref, toRaw, watch } from "vue";
import { IEndpointConfigItem } from "../../database/endpointConfig";
import EditableInputText from "../EditableInputText.vue";
import Icon from "../Icon.vue";

export interface EndpointItemSettingProps {
  item: IEndpointConfigItem;
}

const props = defineProps<EndpointItemSettingProps>();

const emit = defineEmits(["remove", "update", "update:editMode"]);

const editMode = ref(!props.item.id)

const currentValue = ref(structuredClone(toRaw(props.item)));

watch(props.item, () => resetCurrentValue());

function resetCurrentValue() {
  currentValue.value = structuredClone(toRaw(props.item));
}

function toggleEditMode() {
  editMode.value = !editMode.value;

  resetCurrentValue();

  if (!props.item.id) {
    emit('remove')
  }
}

async function applyUpdate() {
  emit("update", currentValue.value);
  editMode.value = !editMode.value;
}
</script>

<template>
  <div class="endpoint-setting">
    <div class="flex flex-col gap-2 bg-light-3 p-4 rounded">
      <div class="flex items-center">
        <div>
          <EditableInputText :editable="editMode" v-model="currentValue.name" />
        </div>

        <div class="tool-icons flex flex-1 justify-end items-center gap-2">
          <Icon v-if="item.id" class="i-carbon:trash-can cursor-pointer" @click="emit('remove')" />

          <template v-if="editMode">
            <Icon class="i-carbon:close cursor-pointer" @click="toggleEditMode" />
            <Icon class="i-carbon:checkmark cursor-pointer" @click="applyUpdate" />
          </template>
          <template v-else>
            <Icon class="i-carbon:edit cursor-pointer" @click="toggleEditMode" />
          </template>
        </div>
      </div>
      <div class="editable-row">
        <label>Base URL</label>
        <EditableInputText :editable="editMode" v-model="currentValue.baseUrl" />
      </div>
      <div class="editable-row">
        <label>Model</label>
        <EditableInputText :editable="editMode" v-model="currentValue.model" />
      </div>
      <div class="editable-row">
        <label>API Key</label>
        <template v-if="editMode">
          <Password class="content" v-model="currentValue.apiKey" :disabled="!editMode" toggleMask :feedback="false" />
        </template>
        <template v-else>
          *************
        </template>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.tool-icons {
  --uno: transition;
  opacity: 0;
}

.endpoint-setting {
  &:hover {
    .tool-icons {
      opacity: 1;
    }

  }
}


.editable-row {
  --uno: flex items-center;

  label {
    width: 100px;
    text-align: right;
    margin-right: 0.5rem;
  }

  .content {
    width: calc(100% - 150px);

    &:deep(.p-inputtext) {
      width: 100%;
    }
  }
}
</style>
