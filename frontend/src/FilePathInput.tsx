// FilePathInput.tsx
import React, { useState } from 'react';
import { FaSpinner } from 'react-icons/fa';
import { invoke } from '@tauri-apps/api/core';

type FilePathInputProps = {
    onSubmitSuccess: (result: Map<string, string>) => void;
};

const FilePathInput: React.FC<FilePathInputProps> = ({ onSubmitSuccess }) => {
    const [filePath, setFilePath] = useState<string | null>(null);
    const [error, setError] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    const handleFileSelection = async () => {
        try {
            const selectedFilePath = await invoke('open_file_dialog');

            if (selectedFilePath) {
                // @ts-ignore
                setFilePath(selectedFilePath);
                setError('');
            } else {
                setError('No file selected');
            }
        } catch (err) {
            console.error('Error selecting file:', err);
            setError('Failed to select a file');
        }
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!filePath) {
            setError('File path is required');
            return;
        }

        setIsLoading(true);
        try {
            const result = await invoke('submit_command', { path: filePath });
            console.log('Backend response:', result);

            const javaFiles = new Map(Object.entries(result as Record<string, string>));
            onSubmitSuccess(javaFiles);

            setFilePath(null);
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
                <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center">Select a File</h2>

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div className="relative">
                        <label className="block text-sm font-medium text-gray-700 mb-1">File Path</label>
                        <button
                            type="button"
                            onClick={handleFileSelection}
                            className="w-full bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium py-3 px-4 rounded-lg transition-colors duration-200"
                        >
                            {filePath || 'Choose a file'}
                        </button>
                        {error && (
                            <p id="error-message" className="mt-2 text-sm text-red-600" role="alert">{error}</p>
                        )}
                    </div>

                    <button
                        type="submit"
                        disabled={isLoading || !filePath}
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
