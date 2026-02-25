<script setup lang="ts">
import { NCard, NButton, NSpace, NModal, NForm, NFormItem, NInput, NList, NListItem } from "naive-ui";
import { ref, onMounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

interface GameItem {
  id: string;
  name: string;
  savePath: string;
}

const gameList = ref<GameItem[]>([]);
const showModal = ref(false);
const newGameName = ref("");
const newGameSavePath = ref("");

const STORAGE_KEY = "sl-manager-games";

function loadGames() {
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored) {
    gameList.value = JSON.parse(stored);
  }
}

function saveGames() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(gameList.value));
}

function openModal() {
  newGameName.value = "";
  newGameSavePath.value = "";
  showModal.value = true;
}

async function selectFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择存档文件夹"
  });
  
  if (selected && typeof selected === "string") {
    newGameSavePath.value = selected;
  }
}

function handleConfirm() {
  if (!newGameName.value.trim() || !newGameSavePath.value.trim()) {
    return;
  }
  
  const newGame: GameItem = {
    id: Date.now().toString(),
    name: newGameName.value.trim(),
    savePath: newGameSavePath.value.trim(),
  };
  
  gameList.value.push(newGame);
  saveGames();
  showModal.value = false;
}

onMounted(() => {
  loadGames();
});
</script>

<template>
  <NCard title="游戏列表" bordered>
    <template #header-extra>
      <NButton type="primary" @click="openModal">新建</NButton>
    </template>
    
    <div class="card-content">
      <NList v-if="gameList.length > 0" hoverable clickable>
        <NListItem v-for="game in gameList" :key="game.id">
          <div class="game-item">
            <span class="game-name">{{ game.name }}</span>
            <span class="game-path">{{ game.savePath }}</span>
          </div>
        </NListItem>
      </NList>
      <p v-else class="empty-tip">暂无游戏，请点击"新建"添加</p>
    </div>
  </NCard>

  <NModal v-model:show="showModal" preset="card" title="新建游戏" style="width: 500px;">
    <NForm>
      <NFormItem label="名称">
        <NInput v-model:value="newGameName" placeholder="请输入游戏名称" />
      </NFormItem>
      <NFormItem label="存档地址">
        <div class="path-input-wrapper">
          <NInput v-model:value="newGameSavePath" placeholder="请输入存档地址" />
          <NButton quaternary @click="selectFolder" class="folder-btn">
            <template #icon>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
              </svg>
            </template>
          </NButton>
        </div>
      </NFormItem>
    </NForm>
    <template #footer>
      <NSpace justify="end">
        <NButton @click="showModal = false">取消</NButton>
        <NButton type="primary" @click="handleConfirm">确定</NButton>
      </NSpace>
    </template>
  </NModal>
</template>

<style scoped>
.card-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 100%;
}

.game-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.game-name {
  font-weight: 500;
  font-size: 14px;
}

.game-path {
  font-size: 12px;
  color: #999;
}

.empty-tip {
  color: #999;
  text-align: center;
  padding: 40px 0;
}

.path-input-wrapper {
  display: flex;
  gap: 8px;
  width: 100%;
}

.path-input-wrapper :deep(.n-input) {
  flex: 1;
}

.folder-btn {
  flex-shrink: 0;
}
</style>
