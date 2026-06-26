<script setup lang="ts">
</script>

<template>
    <div class="base-card">
        <slot></slot>
    </div>
</template>

<style scoped>
.base-card {
    height: 100px;
    width: 200px;
    border-radius: 10px;
    background: #e0e0e0;
    box-shadow: 20px 20px 60px #bebebe,
                -20px -20px 60px #ffffff;
    overflow-y: auto;
}
</style>