import React, { useState, ChangeEvent } from 'react';

type JavaEditorTabsProps = {
    javaFiles: Map<string, string>;
    goBack: () => void; // Add goBack prop
};

const JavaEditorTabs: React.FC<JavaEditorTabsProps> = ({ javaFiles, goBack }) => {
    const [activeTab, setActiveTab] = useState<string>(Array.from(javaFiles.keys())[0]);
    const [fileContents, setFileContents] = useState<Map<string, string>>(new Map(javaFiles));

    const handleTabClick = (className: string) => {
        setActiveTab(className);
    };

    const handleEditorChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
        setFileContents(new Map(fileContents.set(activeTab, event.target.value)));
    };

    return (
        <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-blue-50 to-purple-50 p-4">
            <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-8">
                <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center">Java Source File Editor</h2>

                {/* Tabs */}
                <div className="flex overflow-x-auto border-b mb-4 space-x-4">
                    {Array.from(fileContents.keys()).map((className) => (
                        <button
                            key={className}
                            onClick={() => handleTabClick(className)}
                            className={`px-4 py-2 font-semibold text-gray-800 ${activeTab === className ? 'border-b-2 border-blue-500' : ''}`}
                        >
                            {className}
                        </button>
                    ))}
                </div>

                {/* Editor */}
                <div>
                    <textarea
                        value={fileContents.get(activeTab) || ''}
                        onChange={handleEditorChange}
                        className="w-full h-64 p-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                        placeholder={`Edit ${activeTab} code here...`}
                    />
                </div>

                {/* Back Button */}
                <button
                    onClick={goBack}
                    className="mt-4 w-full bg-gray-500 hover:bg-gray-600 text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200"
                >
                    Back to File Input
                </button>
            </div>
        </div>
    );
};

export default JavaEditorTabs;
