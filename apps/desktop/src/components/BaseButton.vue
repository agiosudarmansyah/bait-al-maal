<script setup></script>

<template>
    <button class="btn">
        <slot />
    </button>
</template>

<style scoped>
.btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--bg);
    color: inherit;
    text-decoration: none;
    font-weight: 500;
    border: none;
    border-radius: 0.5rem;
    box-shadow:  20px 20px 60px #bebebe,
                -20px -20px 60px #ffffff;
}
</style>