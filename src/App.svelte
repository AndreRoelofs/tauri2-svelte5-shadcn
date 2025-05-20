<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import HelloWorld from '$lib/components/HelloWorld.svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Card } from '$lib/components/ui/card/index.js';

	interface ContentBlock {
		id: string;
		content: string;
	}

	let contentBlocks = $state<ContentBlock[]>([]);
	let scalingFactor: number | undefined = $state(undefined);
	let mainElement: HTMLElement;

	// Function to add a new content block
	function addContentBlock() {
		const newBlock = {
			id: crypto.randomUUID(),
			content: `Content Block ${contentBlocks.length + 1}`
		};
		contentBlocks = [...contentBlocks, newBlock];

		// After state update, resize the window
		setTimeout(updateWindowSize, 100);
	}

	function clearContentBlocks() {
		contentBlocks = [];
		updateWindowSize();
	}

	// Function to update window size based on content height
	async function updateWindowSize() {
		if (!mainElement || !scalingFactor) return;

		const contentHeight = mainElement.scrollHeight;
		console.log('contentHeight', contentHeight);
		console.log('offsetHeight', mainElement.offsetHeight);
		console.log('clientHeight', mainElement.clientHeight);
		try {
			await invoke('resize_window', {
				height: contentHeight,
				scalingFactor: scalingFactor
			});
		} catch (error) {
			console.error('Failed to resize window:', error);
		}
	}

	// Set up resize observer to monitor content height changes
	onMount(() => {
		invoke('adjust_initial_window_size', { height: mainElement.scrollHeight }).then(
			(sf: unknown) => {
				scalingFactor = sf as number;
				if (mainElement) {
					const resizeObserver = new ResizeObserver(() => {
						updateWindowSize();
					});

					resizeObserver.observe(mainElement);

					return () => {
						resizeObserver.disconnect();
					};
				}
			}
		);
	});
</script>

<main
	bind:this={mainElement}
	class="m-0 flex min-h-[435px] flex-col items-center bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 p-0 select-none"
>
	<HelloWorld />

	<div class="mt-6">
		<Button onclick={addContentBlock} class="bg-gradient-to-r from-blue-500 to-purple-500">
			Add Content Block
		</Button>
		<Button onclick={clearContentBlocks} class="bg-gradient-to-r from-blue-500 to-purple-500">
			Clear Content Blocks
		</Button>
	</div>

	{#if contentBlocks.length > 0}
		<div class="mt-6 flex w-full flex-col items-center gap-4">
			{#each contentBlocks as block (block.id)}
				<Card class="w-[420px] p-4 shadow-md">
					<h3 class="text-lg font-medium">{block.content}</h3>
					<p class="mt-2 text-gray-600">
						This is a dynamically added content block that increases the page height.
					</p>
				</Card>
			{/each}
		</div>
	{/if}
</main>
