import React from 'react';

interface Todo {
    id: string;
    title: string;
    completed: boolean;
    created_at: string;
    updated_at: string;
}

const TodoCard: React.FC<{ todo: Todo }> = ({ todo }) => {
    return (
        <div className="flex-1">
            <p className="text-xs text-gray-300">
                Id: {todo.id}
            </p>
            <h3 className={`font-semibold ${todo.completed ? 'line-through text-gray-500' : 'text-gray-800'}`}>
                {todo.title}
            </h3>
            <p className="text-sm text-gray-600">
                Status: {todo.completed ? '✅ Completed' : '⏳ Pending'}
            </p>
            <p className="text-xs text-gray-500">
                Created: {new Date(todo.created_at).toLocaleDateString()}
            </p>
        </div>
    );
};

export default TodoCard;