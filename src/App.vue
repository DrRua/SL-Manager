<script setup lang="ts">
import GameList from "./components/GameList.vue";
import SaveBackupList from "./components/SaveBackupList.vue";
</script>

<template>
  <div class="app-container">
    <div class="cards-container">
      <GameList />
      <SaveBackupList />
    </div>
  </div>
</template>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: stretch;
  padding: 20px;
}

.cards-container {
  display: flex;
  gap: 20px;
  width: 100%;
  max-width: 1000px;
  height: 100%;
}

.cards-container :deep(.n-card) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.cards-container :deep(.n-card__body) {
  flex: 1;
  overflow: auto;
}

.card-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 100%;
}

@media (max-width: 768px) {
  .cards-container {
    flex-direction: column;
  }
}

</style>