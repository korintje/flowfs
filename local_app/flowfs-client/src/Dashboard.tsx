import * as React from 'react';
import { styled, createTheme, ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import MuiDrawer from '@mui/material/Drawer';
import Box from '@mui/material/Box';
import MuiAppBar, { AppBarProps as MuiAppBarProps } from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import List from '@mui/material/List';
import Typography from '@mui/material/Typography';
import Divider from '@mui/material/Divider';
import IconButton from '@mui/material/IconButton';
import Badge from '@mui/material/Badge';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import MenuIcon from '@mui/icons-material/Menu';
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import NotificationsIcon from '@mui/icons-material/Notifications';
import { mainListItems, secondaryListItems } from './listItems';

import { invoke } from '@tauri-apps/api/tauri';
import MessageGrid from './MessageGrid';
import { SlackMessageProps } from './SlackMessage';

// const invoke = window.__TAURI__.invoke;
// invoke('my_custom_command');

const drawerWidth: number = 240;

interface AppBarProps extends MuiAppBarProps {
  open?: boolean;
}

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== 'open',
})<AppBarProps>(({ theme, open }) => ({
  zIndex: theme.zIndex.drawer + 1,
  transition: theme.transitions.create(['width', 'margin'], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  ...(open && {
    marginLeft: drawerWidth,
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  }),
}));

const Drawer = styled(MuiDrawer, { shouldForwardProp: (prop) => prop !== 'open' })(
  ({ theme, open }) => ({
    '& .MuiDrawer-paper': {
      position: 'relative',
      whiteSpace: 'nowrap',
      width: drawerWidth,
      transition: theme.transitions.create('width', {
        easing: theme.transitions.easing.sharp,
        duration: theme.transitions.duration.enteringScreen,
      }),
      boxSizing: 'border-box',
      ...(!open && {
        overflowX: 'hidden',
        transition: theme.transitions.create('width', {
          easing: theme.transitions.easing.sharp,
          duration: theme.transitions.duration.leavingScreen,
        }),
        width: theme.spacing(7),
        [theme.breakpoints.up('sm')]: {
          width: theme.spacing(9),
        },
      }),
    },
  }),
);

// TODO remove, this demo shouldn't need to reset the theme.
const defaultTheme = createTheme();

export default function Dashboard() {

  /*
  const [cells, setCells] = React.useState<SlackMessageProps[]>();
  */

  const [open, setOpen] = React.useState(true);
  const [cells, setCells] = React.useState();
  invoke('load_cells').then((cs) => setCells(cs));
  const toggleDrawer = () => { setOpen(!open); };

  const slack_messages = [
    {
      userAvatar: '',
      userName: 'Suzuki',
      messageText: "Hello world!: unable to create directory '/run/user/1000/dconf': Permission denied. dconf will not work properly.",
      attachedFile: {
        name: "Attachments",
        files: [
          { name: "file1.jpg", url: "https://storage.googleapis.com/zenn-user-upload/avatar/58e5499c30.jpeg" },
          { name: "file2.png", url: "https://static.zenn.studio/images/empty/user-content.png" }
        ],
        dirs: [
          {
            name: "Attachments2",
            files: [
              { name: "shoro.pdf", url: "https://www.jstage.jst.go.jp/article/jila/81/5/81_577/_pdf" },
              { name: "file2.png", url: "https://static.zenn.studio/images/empty/user-content.png" }
            ],
            dirs: [],
          }
        ],
      },
    },
    // 他のメッセージも追加できます
    {
      userAvatar: '',
      userName: 'Suzuki',
      messageText: "Hello world!: unable to create directory '/run/user/1000/dconf': Permission denied. dconf will not work properly.",
      attachedFile: {
        name: "Attachments",
        files: [
          { name: "file1.jpg", url: "https://storage.googleapis.com/zenn-user-upload/avatar/58e5499c30.jpeg" },
          { name: "file2.png", url: "https://static.zenn.studio/images/empty/user-content.png" }
        ],
        dirs: [
          {
            name: "Attachments2",
            files: [
              { name: "shoro.pdf", url: "https://www.jstage.jst.go.jp/article/jila/81/5/81_577/_pdf" },
              { name: "file2.png", url: "https://static.zenn.studio/images/empty/user-content.png" }
            ],
            dirs: [],
          }
        ],
      },
    },
  ];

  return (
    <ThemeProvider theme={defaultTheme}>
      <Box sx={{ display: 'flex' }}>
        <CssBaseline />
        <AppBar position="absolute" open={open}>
          <Toolbar
            sx={{
              pr: '24px', // keep right padding when drawer closed
            }}
          >
            <IconButton
              edge="start"
              color="inherit"
              aria-label="open drawer"
              onClick={toggleDrawer}
              sx={{
                marginRight: '36px',
                ...(open && { display: 'none' }),
              }}
            >
              <MenuIcon />
            </IconButton>
            <Typography
              component="h1"
              variant="h6"
              color="inherit"
              noWrap
              sx={{ flexGrow: 1 }}
            >
              Dashboard
            </Typography>
            <IconButton color="inherit">
              <Badge badgeContent={4} color="secondary">
                <NotificationsIcon />
              </Badge>
            </IconButton>
          </Toolbar>
        </AppBar>
        <Drawer variant="permanent" open={open}>
          <Toolbar
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'flex-end',
              px: [1],
            }}
          >
            <IconButton onClick={toggleDrawer}>
              <ChevronLeftIcon />
            </IconButton>
          </Toolbar>
          <Divider />
          <List component="nav">
            {mainListItems}
            <Divider sx={{ my: 1 }} />
            {secondaryListItems}
          </List>
        </Drawer>
        <Box
          component="main"
          sx={{
            backgroundColor: (theme) =>
              theme.palette.mode === 'light'
                ? theme.palette.grey[100]
                : theme.palette.grey[900],
            flexGrow: 1,
            height: '100vh',
            overflow: 'auto',
          }}
        >
          <Toolbar />
          <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
            <Grid container spacing={3}>
              <MessageGrid slack_messages={slack_messages} />
            </Grid>
          </Container>
        </Box>
      </Box>
    </ThemeProvider>
  );
}
