<script setup lang="ts">
</script>

<template>
    <Teleport>
        <div>
            <header>
                <slot></slot>
            </header>
            <main>
                <slot></slot>
            </main>
            <footer>
                <slot></slot>
            </footer>
        </div>
    </Teleport>
</template>

<style scoped>
.modal {
    display: none;
    height: 200px;
    width: 500px;
}
</style>