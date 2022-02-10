<template>
	<LayoutRow class="color-input">
		<LayoutRow class="color-box" :class="{ disabled }" @click="() => clickColorBox()" data-hover-menu-spawner></LayoutRow>
	</LayoutRow>
</template>

<style lang="scss">
.color-input {
	position: relative;
	flex: 0 0 auto;

	.color-box {
		min-width: 80px;
		height: 24px;
		border-radius: 2px;
		background: cyan;
		border: 1px solid var(--color-7-middlegray);
		box-sizing: border-box;
	}
}
</style>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import LayoutRow from "@/components/layout/LayoutRow.vue";
import MenuList from "@/components/widgets/floating-menus/MenuList.vue";

export default defineComponent({
	emits: ["update:value"],
	props: {
		value: { type: String as PropType<string>, required: true },
		disabled: { type: Boolean as PropType<boolean>, default: false },
	},
	computed: {
		color: {
			get() {
				return this.value;
			},
			set(value: string) {
				this.$emit("update:value", value);
			},
		},
	},
	methods: {
		clickColorBox() {
			if (!this.disabled) (this.$refs.menuList as typeof MenuList).setOpen();
		},
	},
	components: {
		LayoutRow,
	},
});
</script>
