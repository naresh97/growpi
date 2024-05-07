<script>
import Button from 'primevue/button';
import InputNumber from 'primevue/inputnumber';
import ToggleButton from 'primevue/togglebutton';
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
    };
  },
  components: { ToggleButton, Button, InputNumber },
  methods: {
    async updateInfo() {
      let info = await receiveInfo();
      this.light_state = to_bool(info.light_state);
      this.fan_state = to_bool(info.fan_state);
      this.pump_state = to_bool(info.pump_state);
      this.temperature = info.temperature;
      this.soil_moisture = info.soil_moisture;
    },
    async sendSwitch(name, state) {
      await fetch("/api/switch/" + name + "/" + to_state(state));
      await this.updateInfo();
    },
    async pumpWater() {
      console.log("Pumping: " + this.pumpAmount);
      await fetch("/api/pump/" + this.pumpAmount);
      await this.updateInfo();
    }
  },
  created() {
    this.updateInfo();
    setInterval(this.updateInfo, 1000);
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