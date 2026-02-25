<script setup lang="ts">
import { NCard, NButton, NSpace, NModal, NForm, NFormItem, NInput, NList, NListItem, NDropdown, useDialog } from "naive-ui";
import { ref, onMounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

interface GameItem {
  id: string;
  name: string;
  savePath: string;
}

const emit = defineEmits<{
  (e: 'gameSelected', game: GameItem | null): void
}>();

const gameList = ref<GameItem[]>([]);
const selectedGameId = ref<string | null>(null);
const showModal = ref(false);
const isEditMode = ref(false);
const editingGameId = ref<string | null>(null);
const newGameName = ref("");
const newGameSavePath = ref("");
const dropdownShow = ref(false);
const dialog = useDialog();

const STORAGE_KEY = "sl-manager-games";
const SELECTED_GAME_KEY = "sl-manager-selected-game";

const dropdownOptions = [
  { label: "编辑", key: "edit" },
  { label: "删除", key: "delete" }
];

function loadGames() {
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored) {
    gameList.value = JSON.parse(stored);
  }
  
  const storedSelected = localStorage.getItem(SELECTED_GAME_KEY);
  if (storedSelected) {
    const exists = gameList.value.some(g => String(g.id) === storedSelected);
    if (exists) {
      selectedGameId.value = storedSelected;
      const selectedGame = gameList.value.find(g => g.id === selectedGameId.value) || null;
      emit('gameSelected', selectedGame);
    }
  }
}

function saveGames() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(gameList.value));
}

function saveSelectedGame() {
  if (selectedGameId.value) {
    localStorage.setItem(SELECTED_GAME_KEY, selectedGameId.value);
  } else {
    localStorage.removeItem(SELECTED_GAME_KEY);
  }
}

function openNewModal() {
  editingGameId.value = null;
  newGameName.value = "";
  newGameSavePath.value = "";
  isEditMode.value = false;
  showModal.value = true;
}

function openEditModal(game: GameItem) {
  editingGameId.value = game.id;
  newGameName.value = game.name;
  newGameSavePath.value = game.savePath;
  isEditMode.value = true;
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
  
  if (isEditMode.value && editingGameId.value) {
    const index = gameList.value.findIndex(g => g.id === editingGameId.value);
    if (index !== -1) {
      gameList.value[index].name = newGameName.value.trim();
      gameList.value[index].savePath = newGameSavePath.value.trim();
    }
  } else {
    const newGame: GameItem = {
      id: Date.now().toString(),
      name: newGameName.value.trim(),
      savePath: newGameSavePath.value.trim(),
    };
    gameList.value.push(newGame);
  }
  
  saveGames();
  showModal.value = false;
}

function handleSelectGame(gameId: string) {
  selectedGameId.value = selectedGameId.value === gameId ? null : gameId;
  saveSelectedGame();
  
  const selectedGame = selectedGameId.value 
    ? gameList.value.find(g => g.id === selectedGameId.value) || null
    : null;
  emit('gameSelected', selectedGame);
}

function handleDropdownSelect(key: string, game: GameItem) {
  if (key === "edit") {
    openEditModal(game);
  } else if (key === "delete") {
    dialog.warning({
      title: "确认删除",
      content: `确定要删除"${game.name}"吗？`,
      positiveText: "确定",
      negativeText: "取消",
      onPositiveClick: () => {
        deleteGame(game.id);
      }
    });
  }
  dropdownShow.value = false;
}

function deleteGame(gameId: string) {
  gameList.value = gameList.value.filter(g => g.id !== gameId);
  if (selectedGameId.value === gameId) {
    selectedGameId.value = null;
    saveSelectedGame();
  }
  saveGames();
}

onMounted(() => {
  loadGames();
});
</script>

<template>
  <NCard title="项目" bordered>
    <template #header-extra>
      <NButton type="primary" @click="openNewModal">新建</NButton>
    </template>
    
    <div class="card-content">
      <NList v-if="gameList.length > 0" hoverable clickable>
        <NListItem 
          v-for="game in gameList" 
          :key="game.id"
          :class="{ 'game-item-selected': selectedGameId === game.id }"
          @click="handleSelectGame(game.id)"
        >
          <div class="game-item-content">
            <div class="game-item-info">
              <span class="game-name">{{ game.name }}</span>
              <span class="game-path">{{ game.savePath }}</span>
            </div>
            <NDropdown 
              trigger="click" 
              :options="dropdownOptions" 
              @select="(key: string) => handleDropdownSelect(key, game)"
            >
              <NButton quaternary circle size="small" @click.stop>
                <template #icon>
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <circle cx="12" cy="5" r="2"></circle>
                    <circle cx="12" cy="12" r="2"></circle>
                    <circle cx="12" cy="19" r="2"></circle>
                  </svg>
                </template>
              </NButton>
            </NDropdown>
          </div>
        </NListItem>
      </NList>
      <p v-else class="empty-tip">暂无游戏，请点击"新建"添加</p>
    </div>
  </NCard>

  <NModal v-model:show="showModal" preset="card" :title="isEditMode ? '编辑游戏' : '新建游戏'" style="width: 500px;">
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

.game-item-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.game-item-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  overflow: hidden;
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

.game-item-selected {
  background-color: rgba(24, 160, 88, 0.1);
  border-radius: 6px;
}
</style>
