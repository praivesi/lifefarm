<template>
    <div class="farm-container">
        <h1>Life Farm</h1>
        <div v-if="loading">로딩 중...</div>
        <div v-if="error" style="color: red;">{{ error }}</div>
        <div v-if="cell_matrix" class="blpt-container">
            <div class="flag-container">
                {{ goal }}
            </div>
            <div class="grass-container">
                <div class="grass-week">
                    <div class="grass-weekday" v-for="weekNum in 7" :key="weekNum">{{ weekdays[weekNum - 1] }}</div>
                </div>
                <div v-for="(week, weekIdx) in cell_matrix" :key="weekIdx" class="grass-row">
                    <div v-for="(day, dayIdx) in week" :key="dayIdx">
                        <div v-if="day.status == 'Pass'" class="grass-cell" :style="{backgroundColor: `rgba(100, 155, 50, 1)`}"></div>
                        <div v-if="day.status == 'Fail'"  class="grass-cell" :style="{backgroundColor: `rgb(255, 0, 0, 1)`}"></div>
                        <div v-if="day.status == 'Padding'" class="grass-cell" :style="{backgroundColor: `rgb(255, 255, 255, 0)`}"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import axios from 'axios';

export default {
    name: 'BluePrintMain',
    setup() {
        const loading = ref(false);
        const error = ref(null);
        const cell_matrix = ref([]);
        const goal = ref("");
        const desc = ref("");
        const weekdays = ref(['S', 'M', 'T', 'W', 'T', 'F', 'S']);

        const chunk = (array, chunk_size) => {
            const result = [];
            for (let i = 0; i < array.length; i += chunk_size) {
                result.push(array.slice(i, i + chunk_size));
            }
            return result;
        };

        const fetchData = async () => {
            loading.value = true;
            error.value = null;

            try {
                const response = await axios.get('https://127.0.0.1:30443/front/blpt/1/cell');
                console.log("response: " + JSON.stringify(response.data));

                cell_matrix.value = chunk(response.data.cells, 7);
                goal.value = response.data.blpt.goal;
                desc.value = response.data.blpt.desc;

                console.log("goal: " + goal.value);
                console.log("desc: " + desc.value);
                
            } catch (err) {
                error.value = '데이터를 불러오는 중 오류가 발생했습니다.';
                console.error(err);
            } finally {
                loading.value = false;
            }
        };

        onMounted(fetchData);

        return {
            cell_matrix,
            goal,
            desc,
            weekdays
        }
    }
}
</script>

<style scoped>
.farm-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.flag-container {
    background-color: burlywood;
    padding: 10px;
    margin-bottom: 5px;
}

.blpt-container {
  display: flex;
  flex-direction: column;
  padding: 5px;
}

.grass-week {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
}

.grass-weekday {
    font-size: 2vh;
}

.grass-container {
    border: 2px solid black; /* 두께 5px */
    padding: 5px;
}

.grass-row {
  display: flex;
}
.grass-cell {
  width: 2vh;
  height: 2vh;
  margin: 2px;
  border: 1px;
}
</style>