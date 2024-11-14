import React, { useState, ChangeEvent } from 'react';

type JavaEditorTabsProps = {
    javaFiles: Map<string, string>;
    goBack: () => void;
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
        <div className="h-screen w-screen flex bg-gradient-to-br from-blue-50 to-purple-50">
            {/* Sidebar */}
            <div className="w-1/4 h-full bg-white shadow-lg p-4 overflow-y-auto">
                <h2 className="text-xl font-bold text-gray-800 mb-4 text-center">Java Files</h2>
                <div className="space-y-2">
                    {Array.from(fileContents.keys()).map((className) => (
                        <button
                            key={className}
                            onClick={() => handleTabClick(className)}
                            className={`w-full text-left px-4 py-2 font-semibold text-gray-800 rounded-lg ${
                                activeTab === className ? 'bg-blue-100 text-blue-700' : 'hover:bg-gray-100'
                            }`}
                        >
                            {className}
                        </button>
                    ))}
                </div>
            </div>

            {/* Editor Section */}
            <div className="flex-grow h-full bg-white rounded-xl shadow-lg p-6 flex flex-col">
                <h2 className="text-3xl font-bold text-gray-800 mb-4 text-center">Java Source File Editor</h2>

                {/* Editor */}
                <div className="flex-grow">
                    <textarea
                        value={fileContents.get(activeTab) || ''}
                        onChange={handleEditorChange}
                        className="w-full h-full p-4 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                        placeholder={`Edit ${activeTab} code here...`}
                    />
                </div>

                {/* Back Button */}
                <button
                    onClick={goBack}
                    className="mt-4 w-full bg-gray-500 hover:bg-gray-600 text-white font-medium py-3 rounded-lg transition-colors duration-200"
                >
                    Back to File Input
                </button>
            </div>
        </div>
    );
};

export default JavaEditorTabs;
