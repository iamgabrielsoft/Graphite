<template>
	<LayoutRow class="icon-label" :class="`size-${icons[icon].size}`">
		<component :is="icon" />
	</LayoutRow>
</template>

<style lang="scss">
.icon-label {
	flex: 0 0 auto;
	fill: var(--color-e-nearwhite);

	&.size-12 {
		width: 12px;
		height: 12px;
	}

	&.size-16 {
		width: 16px;
		height: 16px;
	}

	&.size-24 {
		width: 24px;
		height: 24px;
	}
}
</style>

<script lang="ts">
import { DefineComponent, defineComponent, PropType } from "vue";

import { IconName, IconSize, ICON_LIST } from "@/utilities/icons";

import LayoutRow from "@/components/layout/LayoutRow.vue";

const icons: Record<IconName, { component: DefineComponent; size: IconSize }> = ICON_LIST;

export default defineComponent({
	props: {
		icon: { type: String as PropType<IconName>, required: true },
		gapAfter: { type: Boolean as PropType<boolean>, default: false },
	},
	data() {
		return {
			icons,
		};
	},
	components: {
		LayoutRow,
		...Object.fromEntries(Object.entries(icons).map(([name, data]) => [name, data.component])),
	},
});
</script>
