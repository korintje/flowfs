import React, { useState } from 'react';
import { Avatar, Paper, Typography, Divider, Stack, IconButton, Collapse } from '@mui/material';
import InsertDriveFileIcon from '@mui/icons-material/InsertDriveFile';
import ImageIcon from '@mui/icons-material/Image';
import DescriptionIcon from '@mui/icons-material/Description';
import FolderIcon from '@mui/icons-material/Folder';

interface File {
  name: string;
  url: string;
}

interface Directory {
  name: string;
  files: File[];
  dirs: Directory[];
}

export interface SlackMessageProps {
  userAvatar: string;
  userName: string;
  messageText: string;
  attachedFile?: Directory; // attachedFileはDirectory型のオブジェクトとして定義される
}

const getFileIcon = (fileName: string): JSX.Element => {
  const extension = fileName.split('.').pop()?.toLowerCase();
  switch (extension) {
    case 'jpg':
    case 'jpeg':
    case 'png':
      return <ImageIcon />;
    case 'pdf':
      return <DescriptionIcon />;
    default:
      return <InsertDriveFileIcon />;
  }
};

const SlackMessageUI: React.FC<SlackMessageProps> = ({ userAvatar, userName, messageText, attachedFile }) => {
  const [openDirs, setOpenDirs] = useState<string[]>([]);

  const toggleDir = (dirName: string) => {
    setOpenDirs(openDirs.includes(dirName) ? openDirs.filter(dir => dir !== dirName) : [...openDirs, dirName]);
  };

  const renderFiles = (files: File[]) => {
    return files.map((file, index) => (
      <IconButton key={index} href={file.url} target="_blank" rel="noopener noreferrer">
        {getFileIcon(file.name)}
        <Typography variant="body2" sx={{ ml: 1 }}>
          {file.name}
        </Typography>
      </IconButton>
    ));
  };

  const renderDirectories = (dirs: Directory[]) => {
    return dirs.map((dir, index) => (
      <div key={index}>
        <IconButton onClick={() => toggleDir(dir.name)}>
          <FolderIcon />
          <Typography variant="body2" fontWeight="bold" sx={{ ml: 1 }}>
            {dir.name}
          </Typography>
        </IconButton>
        <Collapse in={openDirs.includes(dir.name)}>
          <Stack direction="column" spacing={1} sx={{ pl: 2 }}>
            {renderFiles(dir.files)}
            {renderDirectories(dir.dirs)}
          </Stack>
        </Collapse>
      </div>
    ));
  };

  return (
    <Paper variant="outlined" sx={{ p: 2, mb: 2 }}>
      <Avatar src={userAvatar} alt={userName} sx={{ mr: 2 }} />
      <div>
        <Typography variant="subtitle1" fontWeight="bold" sx={{ mb: 1 }}>
          {userName}
        </Typography>
        <Typography variant="body1" sx={{ mb: 1 }}>
          {messageText}
        </Typography>
        {attachedFile && (
          <div>
            <Typography variant="subtitle2" sx={{ mt: 1 }}>
              Attached Files:
            </Typography>
            <Divider sx={{ my: 1 }} />
            <Stack direction="column" spacing={1}>
              {renderFiles(attachedFile.files)}
              {renderDirectories(attachedFile.dirs)}
            </Stack>
          </div>
        )}
      </div>
    </Paper>
  );
};

export default SlackMessageUI;