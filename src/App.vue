<script setup lang="ts">
import { ref } from "vue";
import { NConfigProvider, NDialogProvider, NMessageProvider, zhCN, dateZhCN, createDiscreteApi } from "naive-ui";
import GameList from "./components/GameList.vue";
import SaveBackupList from "./components/SaveBackupList.vue";

interface GameItem {
  id: string;
  name: string;
  savePath: string;
}

const selectedGame = ref<GameItem | null>(null);

function handleGameSelected(game: GameItem | null) {
  selectedGame.value = game;
}

const { message } = createDiscreteApi(
  ["message"],
  {
    configProviderProps: {
      locale: zhCN,
      dateLocale: dateZhCN,
    },
  }
);

function showMessage(type: 'success' | 'error' | 'warning' | 'info', content: string) {
  message[type](content);
}
</script>

<template>
  <NConfigProvider :locale="zhCN" :date-locale="dateZhCN">
    <NDialogProvider>
      <NMessageProvider>
        <div class="app-container">
          <div class="cards-container">
            <div class="left-card">
              <GameList @game-selected="handleGameSelected" :show-message="showMessage" />
            </div>
            <div class="right-card">
              <SaveBackupList :selected-game="selectedGame" :show-message="showMessage" />
            </div>
          </div>
        </div>
      </NMessageProvider>
    </NDialogProvider>
  </NConfigProvider>
</template>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: stretch;
  padding: 20px;
  background-color: #f5f5f5;
  overflow: hidden;
  box-sizing: border-box;
}

.cards-container {
  display: flex;
  gap: 20px;
  width: 100%;
  height: calc(100% - 0px);
}

.left-card {
  flex: 0 0 auto;
  width: 30%;
  max-width: 400px;
  height: 100%;
}

.right-card {
  flex: 1;
  height: 100%;
}

.cards-container :deep(.n-card) {
  height: 100%;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.cards-container :deep(.n-card__header) {
  border-bottom: 1px solid #e8e8e8;
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
  
  .left-card,
  .right-card {
    flex: 0 0 100%;
  }
}

</style>