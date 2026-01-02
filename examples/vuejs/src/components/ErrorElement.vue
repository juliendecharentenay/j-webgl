<template>
  <div class="fixed inset-0 w-full h-full bg-gray-600/40 flex flex-col justify-center items-center">
    <div class="rounded-md bg-red-500 p-2 flex flex-col">
      <h1>Error:</h1>
      <div>{{ error.msg }}</div>
      <div>{{ error.e }}</div>
      <a href="#" @click="() => {emit('dismiss');}">
        dismiss
      </a>
    </div>
  </div>
</template>
<script setup>
const emit = defineEmits([ "dismiss" ]);
const props = defineProps({
  error: { type: Object, required: true, },
});
</script>
