// App.tsx
import React, { useState } from 'react';
import FilePathInput from './FilePathInput';
import JavaEditorTabs from './JavaEditorTabs';

const App: React.FC = () => {
    const [javaFiles, setJavaFiles] = useState<Map<string, string> | null>(null);

    const handleFileSubmitSuccess = (files: Map<string, string>) => {
        setJavaFiles(files); // Set the retrieved Java files for display
    };

    const goBackToFileInput = () => {
        setJavaFiles(null); // Reset to show FilePathInput again
    };

    return (
        <div>
            {!javaFiles ? (
                <FilePathInput onSubmitSuccess={handleFileSubmitSuccess} />
            ) : (
                <JavaEditorTabs javaFiles={javaFiles} goBack={goBackToFileInput} />
            )}
        </div>
    );
};

export default App;
