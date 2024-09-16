#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>

#define MAX 100

typedef struct {
	int data[MAX];
	int front;
	int rear;
} Queue;

void queue_init(Queue* queue) {
	queue->front = 0;
	queue->rear = 0;
}

void queue_push(Queue* queue, int x) {
	if (queue->rear < MAX) {
		queue->data[queue->rear++] = x;
	}
}

int queue_pop(Queue* queue) {
	if (queue->front != queue->rear) {
		return queue->data[queue->front++];
	}
	return -1;
}

int queue_size(Queue* queue) {
	return queue->rear - queue->front;
}

bool queue_is_empty(Queue* queue) {
	return queue->front == queue->rear;
}

typedef struct {
	Queue queue1;
	Queue queue2;
} MyStack;

void my_stack_init(MyStack* stack) {
	queue_init(&stack->queue1);
	queue_init(&stack->queue2);
}

void my_stack_push(MyStack* stack, int x) {
	queue_push(&stack->queue1, x);
}

int my_stack_pop(MyStack* stack) {
	while (queue_size(&stack->queue1) > 1) {
		queue_push(&stack->queue2, queue_pop(&stack->queue1));
	}

	int top_element = queue_pop(&stack->queue1);

	Queue temp = stack->queue1;
	stack->queue1 = stack->queue2;
	stack->queue2 = temp;

	return top_element;
}

int my_stack_top(MyStack* stack) {
	while (queue_size(&stack->queue1) > 1) {
		queue_push(&stack->queue2, queue_pop(&stack->queue1));
	}

	int top_element = queue_pop(&stack->queue1);
	queue_push(&stack->queue2, top_element);

	Queue temp = stack->queue1;
	stack->queue1 = stack->queue2;
	stack->queue2 = temp;

	return top_element;
}

bool my_stack_is_empty(MyStack* stack) {
	return queue_is_empty(&stack->queue1);
}

void test_my_stack_push_and_top() {
	MyStack stack;
	my_stack_init(&stack);

	my_stack_push(&stack, 10);
	my_stack_push(&stack, 20);
	assert(my_stack_top(&stack) == 20);

	my_stack_push(&stack, 30);
	assert(my_stack_top(&stack) == 30);
}

void test_my_stack_pop() {
	MyStack stack;
	my_stack_init(&stack);

	my_stack_push(&stack, 10);
	my_stack_push(&stack, 20);
	my_stack_push(&stack, 30);

	assert(my_stack_pop(&stack) == 30);
	assert(my_stack_top(&stack) == 20);

	assert(my_stack_pop(&stack) == 20);
	assert(my_stack_top(&stack) == 10);

	assert(my_stack_pop(&stack) == 10);
}

void test_my_stack_is_empty() {
	MyStack stack;
	my_stack_init(&stack);

	assert(my_stack_is_empty(&stack) == true);

	my_stack_push(&stack, 1);
	assert(my_stack_is_empty(&stack) == false);

	my_stack_pop(&stack);
	assert(my_stack_is_empty(&stack) == true);
}

void test_my_stack_mixed_operations() {
	MyStack stack;
	my_stack_init(&stack);

	assert(my_stack_is_empty(&stack) == true);

	my_stack_push(&stack, 5);
	my_stack_push(&stack, 15);
	my_stack_push(&stack, 25);
	assert(my_stack_top(&stack) == 25);

	assert(my_stack_pop(&stack) == 25);
	assert(my_stack_top(&stack) == 15);

	my_stack_push(&stack, 35);
	assert(my_stack_top(&stack) == 35);

	assert(my_stack_pop(&stack) == 35);
	assert(my_stack_top(&stack) == 15);
	assert(my_stack_pop(&stack) == 15);
	assert(my_stack_pop(&stack) == 5);

	assert(my_stack_is_empty(&stack) == true);
}

int main() {
	test_my_stack_push_and_top();
	test_my_stack_pop();
	test_my_stack_is_empty();
	test_my_stack_mixed_operations();

	printf("All tests passed successfully.\n");

	return 0;
}



