// FilePathInput.tsx
import React, { useState } from 'react';
import { FaSpinner } from 'react-icons/fa';
import { invoke } from '@tauri-apps/api/core';

type FilePathInputProps = {
    onSubmitSuccess: (result: Map<string, string>) => void;
};

const FilePathInput: React.FC<FilePathInputProps> = ({ onSubmitSuccess }) => {
    const [filePath, setFilePath] = useState('');
    const [error, setError] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    const validateFilePath = (path: string): boolean => {
        const regex = /^([a-zA-Z]:\\[^*|"<>?\n]*)|(\\\\[^*|"<>?\n]*)/;
        return regex.test(path);
    };

    const handleFilePathChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const path = e.target.value;
        setFilePath(path);

        if (!path) {
            setError('File path is required');
        } else if (!validateFilePath(path)) {
            setError('Invalid file path format');
        } else {
            setError('');
        }
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!filePath) {
            setError('File path is required');
            return;
        }

        if (!validateFilePath(filePath)) {
            setError('Invalid file path format');
            return;
        }

        setIsLoading(true);
        try {
            const result = await invoke('submit_controller', { path: filePath });
            console.log('Backend response:', result);

            // Assuming `result` is in a suitable format (e.g., a JSON object),
            // parse it into a Map of Java class names and their source codes
            const javaFiles = new Map(Object.entries(result as Record<string, string>));
            onSubmitSuccess(javaFiles); // Pass the parsed data up to the parent component

            setFilePath('');
        } catch (err) {
            console.error('Error submitting file path:', err);
            setError('Failed to submit file path');
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-purple-50 p-4">
            <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-8">
                <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center">Enter File Path</h2>

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div className="relative">
                        <label htmlFor="filePath" className="block text-sm font-medium text-gray-700 mb-1">File Path</label>
                        <input
                            type="text"
                            id="filePath"
                            value={filePath}
                            onChange={handleFilePathChange}
                            className={`block w-full px-4 py-3 rounded-lg border ${error ? 'border-red-500' : 'border-gray-300'} focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200`}
                            placeholder="Enter full file path"
                            aria-label="File path input"
                            aria-invalid={error ? 'true' : 'false'}
                            aria-describedby={error ? 'error-message' : undefined}
                        />
                        {error && (
                            <p id="error-message" className="mt-2 text-sm text-red-600" role="alert">{error}</p>
                        )}
                    </div>

                    <button
                        type="submit"
                        disabled={isLoading}
                        className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center disabled:opacity-70 disabled:cursor-not-allowed"
                    >
                        {isLoading ? (
                            <>
                                <FaSpinner className="animate-spin mr-2" />
                                Processing...
                            </>
                        ) : (
                            'Submit'
                        )}
                    </button>
                </form>
            </div>
        </div>
    );
};

export default FilePathInput;
