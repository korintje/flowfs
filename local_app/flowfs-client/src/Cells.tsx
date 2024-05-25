import React from 'react';
import { Container, Grid, Paper } from '@mui/material';
import Cell, { CellProps } from './Cell';

export interface CellsProps {
  cells: CellProps[];
}

export const CellsGrid: React.FC<CellsProps> = ({ cells }) => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Grid container spacing={3}>
        {cells.map((cell, index) => (
          <Grid item xs={12} key={index}>
            <Paper sx={{ p: 2, display: 'flex', flexDirection: 'column' }}>
              <Cell
                userAvatar={cell.userAvatar}
                userName={cell.userName}
                messageText={cell.messageText}
                attachedFile={cell.attachedFile}
              />
            </Paper>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
}

// export default Cells;
