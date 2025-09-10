import {
  Box,
  Button,
  IconButton,
  Select,
  MenuItem,
  Typography,
  Stack,
  useTheme,
  useMediaQuery,
} from '@mui/material';
import {
  FirstPage as FirstPageIcon,
  KeyboardArrowLeft as PrevIcon,
  KeyboardArrowRight as NextIcon,
  LastPage as LastPageIcon,
} from '@mui/icons-material';

export interface PaginationProps {
  /**
   * Current page number (1-indexed)
   */
  page: number;
  
  /**
   * Total number of items
   */
  total: number;
  
  /**
   * Number of items per page
   */
  limit: number;
  
  /**
   * Available page size options
   * @default [10, 25, 50, 100]
   */
  limitOptions?: number[];
  
  /**
   * Callback when page changes
   */
  onPageChange: (page: number) => void;
  
  /**
   * Callback when limit changes
   */
  onLimitChange: (limit: number) => void;
  
  /**
   * Whether to show first/last page buttons
   * @default true
   */
  showFirstLastButtons?: boolean;
  
  /**
   * Whether to show page size selector
   * @default true
   */
  showPageSizeSelector?: boolean;
  
  /**
   * Whether to show page info text
   * @default true
   */
  showPageInfo?: boolean;
  
  /**
   * Custom text for items per page
   */
  itemsPerPageText?: string;
  
  /**
   * Custom text for page info
   */
  pageInfoText?: string;
}

const Pagination: React.FC<PaginationProps> = ({
  page,
  total,
  limit,
  limitOptions = [10, 25, 50, 100],
  onPageChange,
  onLimitChange,
  showFirstLastButtons = true,
  showPageSizeSelector = true,
  showPageInfo = true,
  itemsPerPageText = 'Items per page:',
  pageInfoText = 'Page {page} of {pages}',
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
  
  // Calculate total pages
  const totalPages = Math.ceil(total / limit);
  
  // Calculate start and end item indices
  const startItem = total > 0 ? (page - 1) * limit + 1 : 0;
  const endItem = Math.min(page * limit, total);
  
  // Handle page changes
  const handleFirstPage = () => onPageChange(1);
  const handlePrevPage = () => onPageChange(Math.max(1, page - 1));
  const handleNextPage = () => onPageChange(Math.min(totalPages, page + 1));
  const handleLastPage = () => onPageChange(totalPages);
  
  // Handle limit change
  const handleLimitChange = (event: any) => {
    const newLimit = parseInt(event.target.value, 10);
    onLimitChange(newLimit);
    onPageChange(1); // Reset to first page when limit changes
  };
  
  // Generate page numbers to display
  const getPageNumbers = () => {
    const delta = isMobile ? 1 : 2; // Show fewer pages on mobile
    const range: (number | 'ellipsis')[] = [];
    
    // Always show first page
    range.push(1);
    
    // Show ellipsis if needed
    if (page - delta > 2) {
      range.push('ellipsis');
    }
    
    // Show pages around current page
    for (let i = Math.max(2, page - delta); i <= Math.min(totalPages - 1, page + delta); i++) {
      range.push(i);
    }
    
    // Show ellipsis if needed
    if (page + delta < totalPages - 1) {
      range.push('ellipsis');
    }
    
    // Always show last page if there's more than one page
    if (totalPages > 1) {
      range.push(totalPages);
    }
    
    return range;
  };
  
  // Render page number buttons
  const renderPageButtons = () => {
    if (totalPages <= 1) return null;
    
    return getPageNumbers().map((pageNum, index) => {
      if (pageNum === 'ellipsis') {
        return (
          <Button
            key={`ellipsis-${index}`}
            disabled
            sx={{ 
              minWidth: '30px',
              padding: '6px 0',
              fontWeight: 'normal',
            }}
          >
            …
          </Button>
        );
      }
      
      return (
        <Button
          key={pageNum}
          onClick={() => onPageChange(pageNum)}
          variant={pageNum === page ? 'contained' : 'outlined'}
          sx={{
            minWidth: '30px',
            padding: '6px 0',
            fontWeight: pageNum === page ? 'bold' : 'normal',
          }}
        >
          {pageNum}
        </Button>
      );
    });
  };
  
  if (total === 0) {
    return (
      <Box sx={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center',
        py: 2,
      }}>
        <Typography variant="body2" color="textSecondary">
          No items found
        </Typography>
      </Box>
    );
  }
  
  return (
    <Box sx={{ 
      display: 'flex', 
      justifyContent: 'space-between', 
      alignItems: 'center',
      py: 2,
      flexWrap: 'wrap',
      gap: 2,
    }}>
      {/* Page size selector */}
      {showPageSizeSelector && (
        <Stack 
          direction="row" 
          spacing={1} 
          alignItems="center"
          sx={{ 
            minWidth: '150px',
          }}
        >
          <Typography variant="body2" color="textSecondary">
            {itemsPerPageText}
          </Typography>
          <Select
            value={limit}
            onChange={handleLimitChange}
            size="small"
            sx={{ 
              minWidth: '80px',
            }}
          >
            {limitOptions.map((option) => (
              <MenuItem key={option} value={option}>
                {option}
              </MenuItem>
            ))}
          </Select>
        </Stack>
      )}
      
      {/* Page info */}
      {showPageInfo && (
        <Typography variant="body2" color="textSecondary">
          {pageInfoText
            .replace('{page}', page.toString())
            .replace('{pages}', totalPages.toString())}{' '}
          ({startItem}-{endItem} of {total})
        </Typography>
      )}
      
      {/* Navigation controls */}
      <Stack 
        direction="row" 
        spacing={1} 
        alignItems="center"
      >
        {showFirstLastButtons && (
          <IconButton
            onClick={handleFirstPage}
            disabled={page === 1}
            size="small"
            aria-label="First page"
          >
            <FirstPageIcon />
          </IconButton>
        )}
        
        <IconButton
          onClick={handlePrevPage}
          disabled={page === 1}
          size="small"
          aria-label="Previous page"
        >
          <PrevIcon />
        </IconButton>
        
        {/* Page numbers */}
        {renderPageButtons()}
        
        <IconButton
          onClick={handleNextPage}
          disabled={page === totalPages}
          size="small"
          aria-label="Next page"
        >
          <NextIcon />
        </IconButton>
        
        {showFirstLastButtons && (
          <IconButton
            onClick={handleLastPage}
            disabled={page === totalPages}
            size="small"
            aria-label="Last page"
          >
            <LastPageIcon />
          </IconButton>
        )}
      </Stack>
    </Box>
  );
};

export default Pagination;