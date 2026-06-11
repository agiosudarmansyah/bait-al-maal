<script setup lang="ts">
import { ref } from 'vue';
import { RouterLink } from 'vue-router';

const isActive = ref(false);
const { destinationPath } = defineProps<{ destinationPath: string }>();
</script>

<template>
    <RouterLink 
    @click="isActive = !isActive"
    :class="{'nav_button-active': isActive}"
    class = "nav_button" 
    :to="destinationPath">
        <slot></slot>
    </RouterLink>
</template>

<style scoped>
.nav_button {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    height: 100%;
    width: 100%;
    padding-left: 0.25rem;
    color: inherit;
    font-weight: 500;
    text-decoration: none;
}
.nav_button-active {
    border-radius: 10px;
    background: linear-gradient(145deg, #cacaca, #f0f0f0);
    box-shadow: 20px 20px 60px #bebebe,
                -20px -20px 60px #ffffff;
}
</style>