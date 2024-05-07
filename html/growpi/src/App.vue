<script>
import Button from 'primevue/button';
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
    };
  },
  components: { ToggleButton, Button },
  methods: {
    async updateInfo() {
      let info = await receiveInfo();
      this.light_state = to_bool(info.light_state);
      this.fan_state = to_bool(info.fan_state);
      this.pump_state = to_bool(info.pump_state);
      this.temperature = info.temperature;
      this.soil_moisture = info.soil_moisture;
      console.log(info);
    },
    async sendSwitch(name, state) {
      await fetch("http://192.168.0.107:2205/switch/" + name + "/" + to_state(state));
      await this.updateInfo();
    }
  },
  created() {
    setTimeout(this.updateInfo, 1000);
  }
}

async function switchDevice(state) {
  await fetch("http://192.168.0.107:2205/info")
}

async function receiveInfo() {
  const response = await fetch("http://192.168.0.107:2205/info");
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
  <table>
    <tr>
      <td>Lights</td>
      <td>
        <ToggleButton :modelValue="light_state" something="hi" @change="sendSwitch('lights', !light_state)" onLabel="On"
          offLabel="Off" />
      </td>
    </tr>
    <tr>
      <td>Fan</td>
      <td>
        <ToggleButton :modelValue="fan_state" @change="sendSwitch('fan', !fan_state)" onLabel="On" offLabel="Off" />
      </td>
    </tr>
    <tr>
      <td>Pump</td>
      <td>
        <ToggleButton v-model="water_primed" onLabel="Ready" offLabel="Click to Prime" />
        <Button @change="sendSwitch('pump', !water_state); water_primed = false;" :disabled="!water_primed"
          label="Pump Water" />
        {{ water_state ? "Pump active" : "Pump inactive" }}
      </td>
    </tr>
  </table>
</template>
<style></style>