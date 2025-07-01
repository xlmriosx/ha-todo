// components/TodoInterface.tsx
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import TodoCard from './TodoCard';

interface Todo {
  id: string;
  title: string;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

interface ApiResponse<T> {
  status: string;
  data: T;
  error?: string;
}

interface TodoInterfaceProps {
  backendName: string;
}

const TodoInterface: React.FC<TodoInterfaceProps> = ({ backendName }) => {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTodo, setNewTodo] = useState({ title: '', completed: false });
  const [updateTodo, setUpdateTodo] = useState({ id: '', title: '', completed: false });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Define styles based on the backend name
  const backgroundColors: { [key: string]: string } = {
    rust: 'bg-orange-500',
  };

  const buttonColors: { [key: string]: string } = {
    rust: 'bg-orange-700 hover:bg-orange-600',
  };

  const bgColor = backgroundColors[backendName as keyof typeof backgroundColors] || 'bg-gray-200';
  const btnColor = buttonColors[backendName as keyof typeof buttonColors] || 'bg-gray-500 hover:bg-gray-600';

  // Fetch todos
  useEffect(() => {
    const fetchTodos = async () => {
      setLoading(true);
      setError(null);
      try {
        const response = await axios.get<ApiResponse<Todo[]>>(`${apiUrl}/api/v1/todos`);
        if (response.data.status === 'success' && response.data.data) {
          setTodos(response.data.data);
        }
      } catch (error) {
        console.error('Error fetching todos:', error);
        setError('Failed to fetch todos');
      } finally {
        setLoading(false);
      }
    };

    fetchTodos();
  }, [apiUrl]);

  // Create a todo
  const createTodo = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!newTodo.title.trim()) return;

    setLoading(true);
    setError(null);
    try {
      const response = await axios.post<ApiResponse<Todo>>(`${apiUrl}/api/v1/todos`, {
        title: newTodo.title,
        completed: newTodo.completed,
      });
      
      if (response.data.status === 'success' && response.data.data) {
        setTodos([response.data.data, ...todos]);
        setNewTodo({ title: '', completed: false });
      }
    } catch (error) {
      console.error('Error creating todo:', error);
      setError('Failed to create todo');
    } finally {
      setLoading(false);
    }
  };

  // Update a todo
  const handleUpdateTodo = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!updateTodo.id || !updateTodo.title.trim()) return;

    setLoading(true);
    setError(null);
    try {
      const response = await axios.put<ApiResponse<Todo>>(
        `${apiUrl}/api/v1/todos/${updateTodo.id}`,
        {
          title: updateTodo.title,
          completed: updateTodo.completed,
        }
      );

      if (response.data.status === 'success' && response.data.data) {
        setTodos(todos.map(todo => 
          todo.id === updateTodo.id ? response.data.data! : todo
        ));
        setUpdateTodo({ id: '', title: '', completed: false });
      }
    } catch (error) {
      console.error('Error updating todo:', error);
      setError('Failed to update todo');
    } finally {
      setLoading(false);
    }
  };

  // Delete a todo
  const deleteTodo = async (todoId: string) => {
    setLoading(true);
    setError(null);
    try {
      await axios.delete(`${apiUrl}/api/v1/todos/${todoId}`);
      setTodos(todos.filter(todo => todo.id !== todoId));
    } catch (error) {
      console.error('Error deleting todo:', error);
      setError('Failed to delete todo');
    } finally {
      setLoading(false);
    }
  };

  // Toggle todo completion
  const toggleTodo = async (todo: Todo) => {
    setLoading(true);
    setError(null);
    try {
      const response = await axios.put<ApiResponse<Todo>>(
        `${apiUrl}/api/v1/todos/${todo.id}`,
        {
          title: todo.title,
          completed: !todo.completed,
        }
      );

      if (response.data.status === 'success' && response.data.data) {
        setTodos(todos.map(t => 
          t.id === todo.id ? response.data.data! : t
        ));
      }
    } catch (error) {
      console.error('Error toggling todo:', error);
      setError('Failed to toggle todo');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className={`todo-interface ${bgColor} ${backendName} w-full max-w-md p-4 my-4 rounded shadow`}>
      <img 
        src={`/${backendName}logo.svg`} 
        alt={`${backendName} Logo`} 
        className="w-20 h-20 mb-6 mx-auto" 
      />
      <h2 className="text-xl font-bold text-center text-white mb-6">
        {`${backendName.charAt(0).toUpperCase() + backendName.slice(1)} Todo API`}
      </h2>

      {error && (
        <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
          {error}
        </div>
      )}

      {loading && (
        <div className="mb-4 p-3 bg-blue-100 border border-blue-400 text-blue-700 rounded">
          Loading...
        </div>
      )}

      {/* Form to add new todo */}
      <form onSubmit={createTodo} className="mb-6 p-4 bg-blue-100 rounded shadow">
        <input
          placeholder="Enter todo title"
          value={newTodo.title}
          onChange={(e) => setNewTodo({ ...newTodo, title: e.target.value })}
          className="mb-2 w-full p-2 border border-gray-300 rounded"
          disabled={loading}
        />
        <label className="flex items-center mb-2">
          <input
            type="checkbox"
            checked={newTodo.completed}
            onChange={(e) => setNewTodo({ ...newTodo, completed: e.target.checked })}
            className="mr-2"
            disabled={loading}
          />
          <span className="text-sm text-gray-700">Mark as completed</span>
        </label>
        <button 
          type="submit" 
          className="w-full p-2 text-white bg-blue-500 rounded hover:bg-blue-600 disabled:bg-gray-400"
          disabled={loading || !newTodo.title.trim()}
        >
          Add Todo
        </button>
      </form>

      {/* Form to update todo */}
      <form onSubmit={handleUpdateTodo} className="mb-6 p-4 bg-green-100 rounded shadow">
        <input
          placeholder="Todo ID"
          value={updateTodo.id}
          onChange={(e) => setUpdateTodo({ ...updateTodo, id: e.target.value })}
          className="mb-2 w-full p-2 border border-gray-300 rounded"
          disabled={loading}
        />
        <input
          placeholder="New title"
          value={updateTodo.title}
          onChange={(e) => setUpdateTodo({ ...updateTodo, title: e.target.value })}
          className="mb-2 w-full p-2 border border-gray-300 rounded"
          disabled={loading}
        />
        <label className="flex items-center mb-2">
          <input
            type="checkbox"
            checked={updateTodo.completed}
            onChange={(e) => setUpdateTodo({ ...updateTodo, completed: e.target.checked })}
            className="mr-2"
            disabled={loading}
          />
          <span className="text-sm text-gray-700">Mark as completed</span>
        </label>
        <button 
          type="submit" 
          className="w-full p-2 text-white bg-green-500 rounded hover:bg-green-600 disabled:bg-gray-400"
          disabled={loading || !updateTodo.id || !updateTodo.title.trim()}
        >
          Update Todo
        </button>
      </form>

      {/* Display todos */}
      <div className="space-y-4">
        {todos.length === 0 ? (
          <p className="text-center text-white">No todos found. Create your first todo!</p>
        ) : (
          todos.map((todo) => (
            <div key={todo.id} className="flex items-center justify-between bg-white p-4 rounded-lg shadow">
              <TodoCard todo={todo} />
              <div className="flex gap-2 ml-4">
                <button
                  onClick={() => toggleTodo(todo)}
                  className={`px-3 py-1 text-sm rounded ${
                    todo.completed 
                      ? 'bg-yellow-500 hover:bg-yellow-600 text-white' 
                      : 'bg-green-500 hover:bg-green-600 text-white'
                  }`}
                  disabled={loading}
                >
                  {todo.completed ? 'Undo' : 'Done'}
                </button>
                <button
                  onClick={() => deleteTodo(todo.id)}
                  className={`${btnColor} text-white py-1 px-3 text-sm rounded disabled:bg-gray-400`}
                  disabled={loading}
                >
                  Delete
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default TodoInterface;