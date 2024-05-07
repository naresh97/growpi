<script>
import Button from 'primevue/button';
import Image from 'primevue/image';
import InputNumber from 'primevue/inputnumber';
import ToggleButton from 'primevue/togglebutton';
import format from 'date-format'
export default {
  data() {
    return {
      temperature: 0.,
      soil_moisture: 0.,
      light_state: false,
      water_state: false,
      water_primed: false,
      fan_state: false,
      pumpAmount: 150,
      image_src: "/image",
      watering_history: [{ time: 1714976340, amount: 150, moisture_before_watering: 0.7490353 }]
    };
  },
  components: { ToggleButton, Button, InputNumber, Image },
  methods: {
    async updateInfo() {
      let info = await receiveInfo();
      this.light_state = to_bool(info.light_state);
      this.fan_state = to_bool(info.fan_state);
      this.pump_state = to_bool(info.pump_state);
      this.temperature = info.temperature;
      this.soil_moisture = info.soil_moisture;

      let response = await fetch("/api/watering_history/10");
      this.watering_history = await response.json();
    },
    async updateImage() {
      this.image_src = "/image#" + new Date().getTime();
    },
    async sendSwitch(name, state) {
      await fetch("/api/switch/" + name + "/" + to_state(state));
      await this.updateInfo();
    },
    async pumpWater() {
      console.log("Pumping: " + this.pumpAmount);
      await fetch("/api/pump/" + this.pumpAmount);
      await this.updateInfo();
    },
    async refreshImage() {
      await fetch("/api/refresh_image");
      this.updateImage();
    },
    formatDate(timestamp) {
      return format.asString('dd-MM-yyyy at hh:mm', new Date(timestamp * 1000));
    }
  },
  created() {
    this.updateInfo();
    this.updateImage();
    setInterval(this.updateInfo, 1000);
    setInterval(this.updateImage, 10000);
  }
}

async function switchDevice(state) {
  await fetch("/api/info")
}

async function receiveInfo() {
  const response = await fetch("/api/info");
  const info = await response.json();
  return info;
}

function to_state(state) {
  if (state) {
    return "On";
  }
  return "Off"
}

function to_bool(state) {
  if (state == "On") {
    return true;
  }
  return false;
}

</script>
<template>
  <h1>GrowPi!</h1>
  <table class="main-table">
    <tr>
      <td>Temperature</td>
      <td>
        {{ Math.round(temperature * 100) / 100 }}°C
      </td>
    </tr>
    <tr>
      <td>Soil Moisture</td>
      <td>
        {{ Math.round(soil_moisture * 1000) / 10 }}%
      </td>
    </tr>
    <tr>
      <td>Lights</td>
      <td>
        <ToggleButton class="whole-cell" :modelValue="light_state" something="hi"
          @change="sendSwitch('lights', !light_state)" onLabel="On" offLabel="Off" />
      </td>
    </tr>
    <tr>
      <td>Fan</td>
      <td>
        <ToggleButton class="whole-cell" :modelValue="fan_state" @change="sendSwitch('fan', !fan_state)" onLabel="On"
          offLabel="Off" />
      </td>
    </tr>
    <tr>
      <td>Pump</td>
      <td>
        <div class="whole-cell">
          <ToggleButton v-model="water_primed" onLabel="Ready" offLabel="Click to Prime" />
          <Button @click="pumpWater(); water_primed = false;" :disabled="!water_primed && !water_state"
            label="Pump Water" />
        </div>
        <br /><br />
        <InputNumber class="whole-cell" v-model="pumpAmount" :allowEmpty="false" inputId="integeronly" suffix=" grams"
          :min="100" :max="300" />
      </td>
    </tr>
    <tr>
      <td colspan="2">
        <Image :src="image_src" />
        <br /><br />
        <Button @click="refreshImage();" label="Refresh Image" />
      </td>
    </tr>
    <tr>
      <td colspan="2">
        <b>Watering History</b>
        <br /><br />
        <table class="main-table">
          <tr v-for="record in watering_history">
            <td>
              {{ formatDate(record.time) }}
            </td>
            <td>
              {{ record.amount }} grams
            </td>
            <td>
              {{ Math.round(record.moisture_before_watering * 100 * 100) / 100 }}% moisture
            </td>
          </tr>
        </table>
      </td>
    </tr>
  </table>
</template>
<style>
html,
body {
  height: 100%;
}

html {
  display: table;
  margin: auto;
}

body {
  display: table-cell;
  vertical-align: middle;
}

.main-table {
  border-collapse: collapse;
}

.main-table td {
  border-style: solid;
  padding: 1em;
  border-width: 1px;
}

.whole-cell {
  width: 100%;
}
</style>