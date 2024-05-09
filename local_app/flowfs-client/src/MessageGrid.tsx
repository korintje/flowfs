import React from 'react';
import { Container, Grid, Paper } from '@mui/material';
import SlackMessage, { SlackMessageProps } from './SlackMessage'; // SlackMessageコンポーネントのインポート先と型定義を指定してください

interface MessageGridProps {
  slack_messages: SlackMessageProps[];
}

const MessageGrid: React.FC<MessageGridProps> = ({ slack_messages }) => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Grid container spacing={3}>
        {slack_messages.map((message, index) => (
          <Grid item xs={12} key={index}>
            <Paper sx={{ p: 2, display: 'flex', flexDirection: 'column' }}>
              <SlackMessage
                userAvatar={message.userAvatar}
                userName={message.userName}
                messageText={message.messageText}
                attachedFile={message.attachedFile}
              />
            </Paper>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
}

export default MessageGrid;
